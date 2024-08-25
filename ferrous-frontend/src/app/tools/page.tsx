"use client";

import React, {useContext, useState} from 'react';
import FerrousNavbar from "@/components/FerrousNavbar";
import MusicPlayer from "@/components/MusicPlayer";
import {ToolsContext} from "@/contexts/ToolsContext";
import {Card, CardBody} from '@nextui-org/card';
import {Skeleton} from '@nextui-org/skeleton';
import {Button} from '@nextui-org/button';
import {Divider} from "@nextui-org/divider";
import {Select, SelectItem} from '@nextui-org/select';

// TODO: check this
function escapePath(path: string): string {
    // Replace backslashes with forward slashes
    return path.replace(/^\\\\\?\\/, '').replace(/\\/g, '/');
}

export default function ToolsPage() {
    const {toolStatus, downloadAndRecheckTool, updateYtDlp} = useContext(ToolsContext)!;
    const [ytDlpUpdateChannel, setYtDlpUpdateChannel] = useState<string>("stable");
    // States to disable buttons
    const [isDownloading, setIsDownloading] = useState<Record<string, boolean>>({});
    const [isUpdating, setIsUpdating] = useState(false);

    const handleDownloadAndRecheck = async (tool: string) => {
        setIsDownloading(prev => ({...prev, [tool]: true}));
        await downloadAndRecheckTool(tool);
        setIsDownloading(prev => ({...prev, [tool]: false}));
    };

    const handleUpdateYtDlp = async () => {
        setIsUpdating(true);
        await updateYtDlp(ytDlpUpdateChannel);
        setIsUpdating(false);
    };

    return (
        <>
            <FerrousNavbar></FerrousNavbar>

            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 p-4">
                {Object.entries(toolStatus || {}).map(([tool, status]) => (
                    <Card
                        key={tool}
                        className={`border-2 ${status.isOk ? 'border-green-500' : 'border-red-500'} transition-colors duration-500`}
                    >
                        <CardBody>
                            <Skeleton isLoaded={!status.isLoading}>
                                <h3 className="text-lg font-bold text-amber-700">{tool.toUpperCase()}</h3>
                                <p>Status: {status.isOk ? 'OK' : 'Not OK'}</p>
                                <p>Version: {status.version || 'Unknown version'}</p>
                                <p>Path: {status.path ? escapePath(status.path) : 'Unknown path'}</p>
                                {status.error && <p className="text-red-500">Error: {status.error}</p>}
                                {!status.isOk && (
                                    <Button
                                        color="primary"
                                        onClick={() => handleDownloadAndRecheck(tool)}
                                        isLoading={isDownloading[tool]}
                                        className="mt-2"
                                    >
                                        {isDownloading[tool] ? 'Downloading...' : 'Download and Recheck'}
                                    </Button>
                                )}
                                {tool === 'yt-dlp' && status.isOk && (
                                    <div className="mt-2">
                                        <Select
                                            label="Update Channel"
                                            value={ytDlpUpdateChannel}
                                            onChange={(e) => setYtDlpUpdateChannel(e.target.value)}
                                            className="mb-2"
                                        >
                                            <SelectItem key="stable" value="stable">Stable</SelectItem>
                                            <SelectItem key="master" value="master">Master</SelectItem>
                                            <SelectItem key="nightly" value="nightly">Nightly</SelectItem>
                                        </Select>
                                        <Button
                                            color="secondary"
                                            onClick={handleUpdateYtDlp}
                                            isLoading={isUpdating}
                                        >
                                            {isUpdating ? 'Updating...' : 'Update yt-dlp'}
                                        </Button>
                                    </div>
                                )}
                            </Skeleton>
                        </CardBody>
                    </Card>
                ))}
            </div>

            <Divider className="my-4"/>

            <MusicPlayer></MusicPlayer>
        </>
    );
}
