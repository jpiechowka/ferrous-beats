"use client";

import React, {useEffect, useState} from "react";
import {Table, TableBody, TableCell, TableColumn, TableHeader, TableRow} from "@nextui-org/table";
import {Card, CardBody} from "@nextui-org/card";
import {Divider} from "@nextui-org/divider";
import {Button} from "@nextui-org/button";
import {useMusicPlayerContext} from "@/contexts/MusicPlayerContext";
import {Skeleton} from "@nextui-org/skeleton";

export default function MusicLibraryTable() {
    const [libraryPath, setLibraryPath] = useState<string>("");
    const [isLoading, setIsLoading] = useState(true);

    const {
        playlist,
        handleUpdatePlaylistContents,
        handlePlay,
    } = useMusicPlayerContext();

    useEffect(() => {
        setIsLoading(true);
        fetch('http://localhost:13337/library/list')
            .then(response => response.json())
            .then(data => {
                setLibraryPath(data.library_dir);
                handleUpdatePlaylistContents(data.files);
                setIsLoading(false);
            })
            .catch(error => {
                console.error('Error fetching library list:', error)
                setIsLoading(false);
            });
    }, [handleUpdatePlaylistContents]);

    return (
        <>
            <Card className="my-4">
                <CardBody>
                    <h2 className="font-semibold">Local library path: {libraryPath}</h2>
                </CardBody>
            </Card>

            <Divider className="my-4"/>

            <Skeleton isLoaded={!isLoading}>
                <Table isStriped aria-label="Music library table">
                    <TableHeader>
                        <TableColumn>#</TableColumn>
                        <TableColumn>File Name</TableColumn>
                        <TableColumn>Extension</TableColumn>
                        <TableColumn>Actions</TableColumn>
                    </TableHeader>
                    <TableBody>
                        {playlist.map((file, index) => {
                            const lastDotIndex = file.lastIndexOf('.');
                            const fileName = lastDotIndex !== -1 ? file.slice(0, lastDotIndex) : file;
                            const fileExtension = lastDotIndex !== -1 ? file.slice(lastDotIndex + 1) : '';

                            return (
                                <TableRow key={index}>
                                    <TableCell>{index + 1}</TableCell>
                                    <TableCell>{fileName}</TableCell>
                                    <TableCell>{fileExtension}</TableCell>
                                    <TableCell>
                                        <Button
                                            color="primary"
                                            onClick={() => handlePlay(file)}
                                        >
                                            Play
                                        </Button>
                                    </TableCell>
                                </TableRow>
                            );
                        })}
                    </TableBody>
                </Table>
            </Skeleton>
        </>
    );
}
