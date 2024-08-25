"use client";

import React, {useState} from 'react';
import FerrousNavbar from "@/components/FerrousNavbar";
import MusicPlayer from "@/components/MusicPlayer";
import {toast} from "sonner";
import {Button} from '@nextui-org/button';
import {Input} from '@nextui-org/input';
import {Card, CardBody} from '@nextui-org/card';
import {Skeleton} from '@nextui-org/skeleton';
import {useMusicPlayerContext} from "@/contexts/MusicPlayerContext";
import {Divider} from "@nextui-org/divider";

interface DownloadAudioRequest {
    audio_url: string;
}

interface CommandExecutionResults {
    command_completed_successfully: boolean;
    exit_code: number | null;
    stdout: string | null;
    stderr: string | null;
}

interface MediaDownloadResponse {
    download_id: string;
    requested_url: string;
    library_dir: string;
    command_execution_results: CommandExecutionResults;
}

export default function DownloaderPage() {
    const [audioUrl, setAudioUrl] = useState('');
    const [isLoading, setIsLoading] = useState(false);
    const [downloadResult, setDownloadResult] = useState<MediaDownloadResponse | null>(null);
    const {handleUpdatePlaylistContents} = useMusicPlayerContext();

    const handleDownload = async () => {
        setIsLoading(true);
        setDownloadResult(null);

        try {
            const response = await fetch('http://localhost:13337/download/audio', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({audio_url: audioUrl} as DownloadAudioRequest),
            });

            if (!response.ok) {
                throw new Error('Network response was not ok');
            }

            const result: MediaDownloadResponse = await response.json();
            toast.success(`${audioUrl} was downloaded successfully`, {
                duration: 7500,
                closeButton: true,
                position: "bottom-center"
            });
            setDownloadResult(result);

            // Refresh the playlist
            const libraryResponse = await fetch('http://localhost:13337/library/list');
            const libraryData = await libraryResponse.json();
            handleUpdatePlaylistContents(libraryData.files);
            toast.success(`Playlist was refreshed with newly downloaded track`, {
                duration: 7500,
                closeButton: true,
                position: "bottom-center"
            });
        } catch (error) {
            console.error('Error downloading audio:', error);
            toast.error(`Failed to download ${audioUrl}: ${error instanceof Error ? error.message : String(error)}`, {
                duration: Infinity,
                important: true,
                closeButton: true,
                position: "bottom-center"
            });
        } finally {
            setIsLoading(false);
        }
    };
    return (
        <>
            <FerrousNavbar/>
            <div className="container mx-auto px-4 py-8">
                <h1 className="text-2xl font-bold mb-4">Download Music with yt-dlp</h1>
                <div className="flex gap-2 mb-4">
                    <Input
                        placeholder="Enter audio URL"
                        value={audioUrl}
                        onChange={(e) => setAudioUrl(e.target.value)}
                    />
                    <Button color="primary" onClick={handleDownload} disabled={isLoading}>
                        Download
                    </Button>
                </div>

                {isLoading && (
                    <Card className="mb-4">
                        <CardBody>
                            <Skeleton className="rounded-lg">
                                <div className="h-24 rounded-lg bg-default-300"></div>
                            </Skeleton>
                        </CardBody>
                    </Card>
                )}

                {downloadResult && (
                    <Card
                        className={`mb-4 border-2 ${downloadResult.command_execution_results.command_completed_successfully ? 'border-green-500' : 'border-red-500'} transition-colors duration-500`}>
                        <CardBody>
                            <h4>Download Result</h4>
                            <p>Download ID: {downloadResult.download_id}</p>
                            <p>Requested URL: {downloadResult.requested_url}</p>
                            <p>Library Directory: {downloadResult.library_dir}</p>
                            <h5 className="mt-2">Command Execution Results:</h5>
                            <p>Success: {downloadResult.command_execution_results.command_completed_successfully ? 'Yes' : 'No'}</p>
                            {downloadResult.command_execution_results.exit_code !== null && (
                                <p>Exit Code: {downloadResult.command_execution_results.exit_code}</p>
                            )}
                            {downloadResult.command_execution_results.stdout && (
                                <p>Standard Output: {downloadResult.command_execution_results.stdout}</p>
                            )}
                            {downloadResult.command_execution_results.stderr && (
                                <p>Standard Error: {downloadResult.command_execution_results.stderr}</p>
                            )}
                        </CardBody>
                    </Card>
                )}
            </div>

            <Divider className="my-4"/>

            <MusicPlayer/>
        </>
    );
}
