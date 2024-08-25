"use client";

import {createContext, ReactNode, useEffect, useState} from "react";
import {toast} from "sonner";

export const ToolsContext = createContext<{
    toolStatus: Record<string, ToolStatus>;
    downloadAndRecheckTool: (tool: string) => Promise<void>;
} | undefined>(undefined);

interface ToolStatus {
    isOk: boolean;
    isLoading: boolean;
    error: string | null;
    path?: string;
    version?: string;
}

interface CommandExecutionResults {
    command_completed_successfully: boolean;
    exit_code: number | null;
    stdout: string | null;
    stderr: string | null;
}

interface ToolStatusResponse {
    path: string;
    executable_version: string | null;
    command_execution_results: CommandExecutionResults
}

interface ToolDownloadResponse {
    download_url: string;
    tools_dir_path: string;
}

export function ToolsProvider({children}: { children: ReactNode }) {
    const [toolStatus, setToolStatus] = useState<Record<string, ToolStatus>>({
        'ffmpeg': {isOk: false, isLoading: true, error: null},
        'chromaprint': {isOk: false, isLoading: true, error: null},
        'yt-dlp': {isOk: false, isLoading: true, error: null},
    });

    const fetchToolStatus = async (tool: string, endpoint: string) => {
        console.debug(`Fetching status for ${tool} using endpoint ${endpoint}`);
        try {
            const response = await fetch(endpoint);
            if (!response.ok) {
                const errorBody = await response.text();
                console.error(`Error fetching ${tool}, status: ${response.status}, error: ${errorBody}`);
                throw new Error(`Error fetching ${tool}, status: ${response.status}, error: ${errorBody}`);
            }
            const json: ToolStatusResponse = await response.json();
            return {
                tool,
                isOk: json.command_execution_results.command_completed_successfully,
                loading: false,
                error: null,
                path: json.path,
                version: json.executable_version,
            }
        } catch (error) {
            return {
                tool,
                isOk: false,
                loading: false,
                error: error instanceof Error ? error.message : String(error),
                path: null,
                version: null,
            }
        }
    };

    const downloadAndRecheckTool = async (tool: string) => {
        try {
            const response = await fetch(`http://localhost:13337/tools/${tool}/download`, {
                method: 'POST',
            });
            if (!response.ok) {
                throw new Error(`Failed to download ${tool}: ${response.statusText}`);
            }
            const downloadResponse: ToolDownloadResponse = await response.json();
            console.log(`${tool} downloaded successfully to ${downloadResponse.tools_dir_path}`);
            toast.success(`${tool.toUpperCase()} downloaded successfully`, {
                duration: 7500,
                closeButton: true,
                position: "bottom-center"
            });

            // Recheck the tool status
            const newStatus = await fetchToolStatus(tool, `http://localhost:13337/tools/${tool}/status`);
            setToolStatus(prevStatus => ({
                ...prevStatus,
                [tool]: {
                    isOk: newStatus.isOk,
                    isLoading: false,
                    error: newStatus.error,
                    path: newStatus.path ?? undefined,
                    version: newStatus.version ?? undefined,
                },
            }));
        } catch (error) {
            console.error(`Error downloading ${tool}:`, error);
            toast.error(`Failed to download ${tool}: ${error instanceof Error ? error.message : String(error)}`, {
                duration: Infinity,
                important: true,
                closeButton: true,
                position: "bottom-center"
            });
        }
    };

    useEffect(() => {
        const fetchAllToolStatuses = async () => {
            const toolsWithEndpoints = [
                {tool: 'ffmpeg', endpoint: 'http://localhost:13337/tools/ffmpeg/status'},
                {tool: 'chromaprint', endpoint: 'http://localhost:13337/tools/chromaprint/status'},
                {tool: 'yt-dlp', endpoint: 'http://localhost:13337/tools/yt-dlp/status'},
            ]

            const results = await Promise.all(
                toolsWithEndpoints.map(({tool, endpoint}) => fetchToolStatus(tool, endpoint))
            );

            const newToolStatus = results.reduce((acc, status) => {
                acc[status.tool] = {
                    isOk: status.isOk,
                    isLoading: status.loading,
                    error: status.error,
                    path: status.path ?? undefined,
                    version: status.version ?? undefined,
                };

                if (status.isOk) {
                    toast.success(`${status.tool.toUpperCase()} is ready to use, version: ${status.version}`, {
                        duration: 7500,
                        closeButton: true,
                        position: "bottom-center"
                    });
                } else {
                    toast.error(
                        `${status.tool.toUpperCase()} is not ready to use, make sure it is downloaded correctly, error: ${status.error}`, {
                            duration: Infinity,
                            important: true,
                            closeButton: true,
                            position: "bottom-center"
                        });
                }
                return acc;
            }, {} as Record<string, ToolStatus>);

            setToolStatus(newToolStatus);
        };

        fetchAllToolStatuses().catch(error => {
            console.error(`Failed to fetch tool statuses: ${error}`);
        });
    }, []);

    return (
        <ToolsContext.Provider value={{toolStatus, downloadAndRecheckTool}}>
            {children}
        </ToolsContext.Provider>
    );
}