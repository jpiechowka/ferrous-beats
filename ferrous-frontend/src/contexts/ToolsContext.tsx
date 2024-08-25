"use client";

import {createContext, ReactNode, useEffect, useState} from "react";

export const ToolsContext = createContext<Record<string, ToolStatus> | undefined>(undefined);

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

export function ToolsProvider({children}: { children: ReactNode }) {
    const [toolStatus, setToolStatus] = useState<Record<string, ToolStatus>>({
        'ffmpeg': {isOk: false, isLoading: true, error: null},
        'chromaprint': {isOk: false, isLoading: true, error: null},
        'yt-dlp': {isOk: false, isLoading: true, error: null},
    });

    useEffect(() => {
        const fetchToolStatus = async (tool: string, endpoint: string) => {
            try {
                const response = await fetch(endpoint);
                if (!response.ok) {
                    console.error(`Error fetching ${tool}, status: ${response.status}, error: ${response.statusText}`);
                    throw new Error(`Error fetching ${tool}, status: ${response.status}, error: ${response.statusText}`);
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
                return acc;
            }, {} as Record<string, ToolStatus>);

            setToolStatus(newToolStatus);
        };

        fetchAllToolStatuses().catch(error => {
            console.error(`Failed to fetch tool statuses: ${error}`);
        });
    }, []);

    return (
        <ToolsContext.Provider value={toolStatus}>
            {children}
        </ToolsContext.Provider>
    );
}