"use client";

import React, {useEffect, useRef, useState} from "react";
import {Table, TableBody, TableCell, TableColumn, TableHeader, TableRow} from "@nextui-org/table";
import {Card, CardBody} from "@nextui-org/card";
import {Divider} from "@nextui-org/divider";
import {Howl} from 'howler';
import {Button} from "@nextui-org/button";
import MusicPlayer from "./MusicPlayer";

const MAX_SHUFFLE_HISTORY_SIZE: number = 32;

export default function MusicLibraryTable() {
    const [libraryPath, setLibraryPath] = useState<string>("");
    const [libraryContents, setLibraryContent] = useState<string[]>([]);
    const [currentTrackName, setCurrentlyPlaying] = useState<string>("");
    const [currentPlayingIndex, setCurrentPlayingIndex] = useState<number>(0);
    const [isMusicPlaying, setIsMusicPlaying] = useState<boolean>(false);
    const [isShuffleOn, setIsShuffleOn] = useState<boolean>(false);
    const [isRepeatOn, setIsRepeatOn] = useState<boolean>(false);
    const [isLiked, setIsLiked] = useState(false);
    const [volume, setVolume] = useState<number>(0.5);
    const [shuffleHistory, setShuffleHistory] = useState<number[]>([]);
    const [shuffleHistorySize, setShuffleHistorySize] = useState(MAX_SHUFFLE_HISTORY_SIZE);
    const soundRef = useRef<Howl | null>(null);

    useEffect(() => {
        fetch('http://localhost:13337/library/list')
            .then(response => response.json())
            .then(data => {
                setLibraryPath(data.library_dir);
                setLibraryContent(data.files);

                const newShuffleHistorySize = data.files.length <= MAX_SHUFFLE_HISTORY_SIZE
                    ? data.files.length - 1
                    : MAX_SHUFFLE_HISTORY_SIZE;
                setShuffleHistorySize(newShuffleHistorySize)
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

        if (isShuffleOn) {
            setShuffleHistory(prev => [index, ...prev].slice(0, shuffleHistorySize));
            console.debug('Shuffle history:', shuffleHistory);
        }

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

    // TODO: Shuffle is not working correctly yet
    const getNextShuffleIndex = () => {
        const availableTracks = libraryContents.length;
        if (availableTracks <= 1) return 0; // If there's only one track or less, always return 0

        const historySize = Math.min(shuffleHistorySize, availableTracks - 1);
        let nextIndex;
        do {
            nextIndex = Math.floor(Math.random() * availableTracks);
        } while (shuffleHistory.slice(0, historySize).includes(nextIndex));
        return nextIndex;
    };

    const handleNext = () => {
        if (libraryContents.length === 0) return;

        let nextIndex: number;
        if (isShuffleOn) {
            nextIndex = getNextShuffleIndex();
        } else {
            nextIndex = (currentPlayingIndex + 1) % libraryContents.length;
        }

        setCurrentPlayingIndex(nextIndex);
        handlePlay(libraryContents[nextIndex]);
    };

    const handlePrevious = () => {
        if (libraryContents.length === 0) return;

        let prevIndex: number;
        if (isShuffleOn) {
            if (shuffleHistory.length > 1) {
                // Remove the current track from history and get the previous one
                const [, newPrevIndex, ...rest] = shuffleHistory;
                prevIndex = newPrevIndex;
                setShuffleHistory([newPrevIndex, ...rest]);
            } else {
                // If there's no previous track in history, get a new random index
                prevIndex = getNextShuffleIndex();
                setShuffleHistory(prev => [prevIndex, ...prev].slice(0, shuffleHistorySize));
            }
        } else {
            prevIndex = (currentPlayingIndex - 1 + libraryContents.length) % libraryContents.length;
        }

        setCurrentPlayingIndex(prevIndex);
        handlePlay(libraryContents[prevIndex]);
    };

    const handleVolumeChange = (value: number | number[]) => {
        const newVolume = Array.isArray(value) ? value[0] : value; // TODO: is this correct?
        setVolume(newVolume);
        Howler.volume(newVolume);
    };

    const toggleShuffle = () => {
        setIsShuffleOn(prev => {
            if (!prev) {
                // When turning shuffle on, reset the history with the current track
                setShuffleHistory([currentPlayingIndex]);
            } else {
                // When turning shuffle off, clear the history
                setShuffleHistory([]);
            }
            return !prev;
        });
    };

    const toggleRepeat = () => {
        setIsRepeatOn(!isRepeatOn);
    };

    const toggleIsLiked = () => {
        setIsLiked(!isLiked);
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
            {currentTrackName && (
                <>
                    <Divider className="my-4"/>

                    <MusicPlayer
                        currentTrackName={currentTrackName}
                        isMusicPlaying={isMusicPlaying}
                        isShuffleOn={isShuffleOn}
                        isRepeatOn={isRepeatOn}
                        isLiked={isLiked}
                        currentVolume={volume}
                        toggleShuffle={toggleShuffle}
                        toggleRepeat={toggleRepeat}
                        toggleIsLiked={toggleIsLiked}
                        handlePrevious={handlePrevious}
                        handlePlayPause={handlePlayPause}
                        handleNext={handleNext}
                        handleVolumeChange={handleVolumeChange}
                    />
                </>
            )}
        </>
    );
}
