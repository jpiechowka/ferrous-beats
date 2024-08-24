"use client";

import React, {useEffect, useRef, useState} from "react";
import {Table, TableBody, TableCell, TableColumn, TableHeader, TableRow} from "@nextui-org/table";
import {Card, CardBody} from "@nextui-org/card";
import {Divider} from "@nextui-org/divider";
import {Howl} from 'howler';
import {Button} from "@nextui-org/button";
import {Slider} from "@nextui-org/slider";
import {HeartIcon} from "./icons/HeartIcon";
import {PauseCircleIcon} from "./icons/PauseCircleIcon";
import {PlayCircleIcon} from "./icons/PlayCircleIcon";
import {RepeatIcon} from "./icons/RepeatIcon";
import {ShuffleIcon} from "./icons/ShuffleIcon";
import {NextIcon} from "@/components/icons/NextIcon";
import {PreviousIcon} from "@/components/icons/PreviousIcon";

export default function MusicLibraryTable() {
    const [libraryPath, setLibraryPath] = useState<string>("");
    const [libraryContents, setLibraryContent] = useState<string[]>([]);
    const [currentlyPlaying, setCurrentlyPlaying] = useState<string>("");
    const [isMusicPlaying, setIsMusicPlaying] = useState<boolean>(false);
    const [currentPlayingIndex, setCurrentPlayingIndex] = useState<number>(0);
    const [isRepeatOn, setIsRepeatOn] = useState<boolean>(false);
    const [isShuffleOn, setIsShuffleOn] = useState<boolean>(false);
    const [volume, setVolume] = useState<number>(0.5);
    const [liked, setLiked] = useState(false);
    const soundRef = useRef<Howl | null>(null);


    useEffect(() => {
        fetch('http://localhost:13337/library/list')
            .then(response => response.json())
            .then(data => {
                setLibraryPath(data.library_dir);
                setLibraryContent(data.files);
            })
            .catch(error => console.error('Error fetching library list:', error));
    }, []);

    const handlePlay = (fileName: string) => {
        if (soundRef.current) {
            soundRef.current.stop();
        }

        soundRef.current = new Howl({
            src: [`http://localhost:13337/library/play/${encodeURIComponent(fileName)}`],
            html5: true,
            volume: volume,
            onplay: () => {
                setIsMusicPlaying(true);
                setCurrentlyPlaying(fileName);
            },
            onend: () => {
                setIsMusicPlaying(false);
                if (isRepeatOn) {
                    soundRef.current?.play();
                } else {
                    handleNext();
                }
            },
            onpause: () => {
                setIsMusicPlaying(false);
            },
            onstop: () => {
                setIsMusicPlaying(false);
                setCurrentlyPlaying("");
            },
            onloaderror: (id, error) => {
                console.error('Error loading audio:', id, error);
                setCurrentlyPlaying("");
            },
            onplayerror: (id, error) => {
                console.error('Error playing audio:', id, error);
                setCurrentlyPlaying("");
            },
        });

        const index = libraryContents.indexOf(fileName);
        setCurrentPlayingIndex(index);

        soundRef.current.play();
    }

    const handlePlayPause = () => {
        if (soundRef.current) {
            if (isMusicPlaying) {
                soundRef.current.pause();
            } else {
                soundRef.current.play();
            }
        }
    };

    const handleNext = () => {
        if (isShuffleOn) {
            const nextIndex = Math.floor(Math.random() * libraryContents.length);
            setCurrentPlayingIndex(nextIndex);
            handlePlay(libraryContents[nextIndex]);
        } else {
            const nextIndex = (currentPlayingIndex + 1) % libraryContents.length;
            setCurrentPlayingIndex(nextIndex);
            handlePlay(libraryContents[nextIndex]);
        }
    };

    const handlePrevious = () => {
        if (isShuffleOn) {
            const prevIndex = Math.floor(Math.random() * libraryContents.length);
            setCurrentPlayingIndex(prevIndex);
            handlePlay(libraryContents[prevIndex]);
        } else {
            const prevIndex = (currentPlayingIndex - 1 + libraryContents.length) % libraryContents.length;
            setCurrentPlayingIndex(prevIndex);
            handlePlay(libraryContents[prevIndex]);
        }
    };

    const handleVolumeChange = (value: number | number[]) => {
        const newVolume = Array.isArray(value) ? value[0] : value; // TODO: is this correct?
        setVolume(newVolume);
        Howler.volume(newVolume);
    };

    const toggleShuffle = () => {
        setIsShuffleOn(!isShuffleOn);
    };

    const toggleRepeat = () => {
        setIsRepeatOn(!isRepeatOn);
    };

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
                    <TableColumn>Actions</TableColumn>
                </TableHeader>
                <TableBody>
                    {libraryContents.map((file, index) => (
                        <TableRow key={index}>
                            <TableCell>{file}</TableCell>
                            <TableCell>
                                <Button
                                    color="primary"
                                    onClick={() => handlePlay(file)}
                                >
                                    Play
                                </Button>
                            </TableCell>
                        </TableRow>
                    ))}
                </TableBody>
            </Table>

            {/* TODO: Create separate component and delete conditional check */}
            {currentlyPlaying && (
                <>
                    <Divider className="my-4"/>

                    <Card
                        isBlurred
                        className="border-none bg-background/60 dark:bg-default-100/50 my-8 mx-8"
                        shadow="sm"
                    >
                        <CardBody>
                            <div className="grid grid-cols-1 gap-4 items-center justify-center">
                                <div className="flex flex-col">
                                    <div className="flex justify-between items-start">
                                        <div className="flex flex-col gap-0">
                                            <h1 className="text-large font-medium">{currentlyPlaying}</h1>
                                        </div>
                                        <Button
                                            isIconOnly
                                            className="text-default-900/60 data-[hover]:bg-foreground/10"
                                            radius="full"
                                            variant="light"
                                            onPress={() => setLiked((v) => !v)}
                                        >
                                            <HeartIcon
                                                className={liked ? "[&>path]:stroke-transparent" : ""}
                                                fill={liked ? "currentColor" : "none"}
                                            />
                                        </Button>
                                    </div>

                                    <div className="flex w-full items-center justify-center mt-4">
                                        <Button
                                            isIconOnly
                                            className="data-[hover]:bg-foreground/10"
                                            radius="full"
                                            variant="light"
                                            onClick={toggleShuffle}
                                        >
                                            <ShuffleIcon size={24} className={isShuffleOn ? "text-primary" : ""}/>
                                        </Button>
                                        <Button
                                            isIconOnly
                                            className="data-[hover]:bg-foreground/10"
                                            radius="full"
                                            variant="light"
                                            onClick={handlePrevious}
                                        >
                                            <PreviousIcon size={24}/>
                                        </Button>
                                        <Button
                                            isIconOnly
                                            className="data-[hover]:bg-foreground/10"
                                            radius="full"
                                            variant="light"
                                            onClick={handlePlayPause}
                                        >
                                            {isMusicPlaying ? <PauseCircleIcon size={54}/> :
                                                <PlayCircleIcon size={54}/>}
                                        </Button>
                                        <Button
                                            isIconOnly
                                            className="data-[hover]:bg-foreground/10"
                                            radius="full"
                                            variant="light"
                                            onClick={handleNext}
                                        >
                                            <NextIcon size={24}/>
                                        </Button>
                                        <Button
                                            isIconOnly
                                            className="data-[hover]:bg-foreground/10"
                                            radius="full"
                                            variant="light"
                                            onClick={toggleRepeat}
                                        >
                                            <RepeatIcon size={24} className={isRepeatOn ? "text-primary" : ""}/>
                                        </Button>
                                    </div>

                                    <div className="flex items-center mt-4 w-full">
                                        <Slider
                                            label="Volume"
                                            size="md"
                                            color="warning"
                                            step={0.05}
                                            maxValue={1}
                                            minValue={0}
                                            value={volume}
                                            onChange={handleVolumeChange}
                                            className="w-full max-w-md mx-auto"
                                        />
                                    </div>

                                </div>
                            </div>
                        </CardBody>
                    </Card>
                </>
            )}
        </>
    );
}
