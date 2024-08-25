"use client";

import React, {useContext} from 'react';
import FerrousNavbar from "@/components/FerrousNavbar";
import MusicPlayer from "@/components/MusicPlayer";
import {ToolsContext} from "@/contexts/ToolsContext";
import {Card, CardBody} from '@nextui-org/card';
import {Skeleton} from '@nextui-org/skeleton';
import {Button} from '@nextui-org/button';

// TODO: check this
function escapePath(path: string): string {
    // Replace backslashes with forward slashes
    return path.replace(/^\\\\\?\\/, '').replace(/\\/g, '/');
}

export default function ToolsPage() {
    const {toolStatus, downloadAndRecheckTool} = useContext(ToolsContext)!;

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
                                        onClick={() => downloadAndRecheckTool(tool)}
                                        className="mt-2"
                                    >
                                        Download and Recheck
                                    </Button>
                                )}
                            </Skeleton>
                        </CardBody>
                    </Card>
                ))}
            </div>

            <MusicPlayer></MusicPlayer>
        </>
    );
}
