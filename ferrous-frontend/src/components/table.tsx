"use client";

import React, {useEffect, useState} from "react";
import {Table, TableBody, TableCell, TableColumn, TableHeader, TableRow} from "@nextui-org/table";
import {Card, CardBody} from "@nextui-org/card";
import {Divider} from "@nextui-org/divider";

export default function MusicLibraryTable() {
    const [libraryPath, setLibraryPath] = useState<string>("");
    const [libraryContents, setLibraryContent] = useState<string[]>([]);

    useEffect(() => {
        fetch('http://localhost:13337/library/list')
            .then(response => response.json())
            .then(data => {
                setLibraryPath(data.library_dir);
                setLibraryContent(data.files);
            })
            .catch(error => console.error('Error fetching library list:', error));
    }, []);

    return (
        <>
            <Card className="my-4">
                <CardBody>
                    <h2 className="font-semibold">Local library path: {libraryPath}</h2>
                </CardBody>
            </Card>

            <Divider className="my-4"/>

            <Table isStriped aria-label="Music library table">
                <TableHeader>
                    <TableColumn>File Name</TableColumn>
                </TableHeader>
                <TableBody>
                    {libraryContents.map((file, index) => (
                        <TableRow key={index}>
                            <TableCell>{file}</TableCell>
                        </TableRow>
                    ))}
                </TableBody>
            </Table>
        </>
    );
}
