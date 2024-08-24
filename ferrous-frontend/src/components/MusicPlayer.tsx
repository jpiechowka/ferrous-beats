"use client";

import React, {FC} from "react";
import {Card, CardBody} from "@nextui-org/card";
import {Button} from "@nextui-org/button";
import {HeartIcon} from "@/components/icons/HeartIcon";
import {ShuffleIcon} from "@/components/icons/ShuffleIcon";
import {PreviousIcon} from "@/components/icons/PreviousIcon";
import {PauseCircleIcon} from "@/components/icons/PauseCircleIcon";
import {PlayCircleIcon} from "@/components/icons/PlayCircleIcon";
import {NextIcon} from "@/components/icons/NextIcon";
import {RepeatIcon} from "@/components/icons/RepeatIcon";
import {Slider} from "@nextui-org/slider";

interface MusicPlayerProps {
    currentTrackName: string;
    isMusicPlaying: boolean;
    isShuffleOn: boolean;
    isRepeatOn: boolean;
    isLiked: boolean;
    currentVolume: number;
    toggleShuffle: () => void;
    toggleRepeat: () => void;
    toggleIsLiked: () => void;
    handlePrevious: () => void;
    handlePlayPause: () => void;
    handleNext: () => void;
    handleVolumeChange: (value: number | number[]) => void;
}

const MusicPlayer: FC<MusicPlayerProps> = (props) => {
    return (
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
                                <h1 className="text-large font-medium">{props.currentTrackName}</h1>
                            </div>
                            <Button
                                isIconOnly
                                className="text-default-900/60 data-[hover]:bg-foreground/10"
                                radius="full"
                                variant="light"
                                onPress={() => props.toggleIsLiked((v) => !v)}
                            >
                                <HeartIcon
                                    className={props.isLiked ? "[&>path]:stroke-transparent" : ""}
                                    fill={props.isLiked ? "currentColor" : "none"}
                                />
                            </Button>
                        </div>

                        <div className="flex w-full items-center justify-center mt-4">
                            <Button
                                isIconOnly
                                className="data-[hover]:bg-foreground/10"
                                radius="full"
                                variant="light"
                                onClick={props.toggleShuffle}
                            >
                                <ShuffleIcon size={24} className={props.isShuffleOn ? "text-primary" : ""}/>
                            </Button>
                            <Button
                                isIconOnly
                                className="data-[hover]:bg-foreground/10"
                                radius="full"
                                variant="light"
                                onClick={props.handlePrevious}
                            >
                                <PreviousIcon size={24}/>
                            </Button>
                            <Button
                                isIconOnly
                                className="data-[hover]:bg-foreground/10"
                                radius="full"
                                variant="light"
                                onClick={props.handlePlayPause}
                            >
                                {props.isMusicPlaying ? <PauseCircleIcon size={54}/> :
                                    <PlayCircleIcon size={54}/>}
                            </Button>
                            <Button
                                isIconOnly
                                className="data-[hover]:bg-foreground/10"
                                radius="full"
                                variant="light"
                                onClick={props.handleNext}
                            >
                                <NextIcon size={24}/>
                            </Button>
                            <Button
                                isIconOnly
                                className="data-[hover]:bg-foreground/10"
                                radius="full"
                                variant="light"
                                onClick={props.toggleRepeat}
                            >
                                <RepeatIcon size={24} className={props.isRepeatOn ? "text-primary" : ""}/>
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
                                value={props.currentVolume}
                                onChange={props.handleVolumeChange}
                                className="w-full max-w-md mx-auto"
                            />
                        </div>

                    </div>
                </div>
            </CardBody>
        </Card>
    );
};

export default MusicPlayer;