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
import {useMusicPlayerContext} from "@/contexts/MusicPlayerContext";

const MusicPlayer: FC = () => {
    const {
        currentTrackName,
        currentVolume,
        isMusicPlaying,
        isShuffleOn,
        isRepeatOn,
        isTrackLiked,
        toggleShuffle,
        toggleRepeat,
        toggleTrackLiked,
        handleVolumeChange,
        handlePlayPause,
        handleNext,
        handlePrevious
    } = useMusicPlayerContext();

    if (!currentTrackName || currentTrackName.trim() === "") {
        return null; // Don't render anything if no track is playing
    }

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
                                <h1 className="text-large font-medium">{currentTrackName}</h1>
                            </div>
                            <Button
                                isIconOnly
                                className="text-default-900/60 data-[hover]:bg-foreground/10"
                                radius="full"
                                variant="light"
                                onPress={toggleTrackLiked}
                            >
                                <HeartIcon
                                    className={isTrackLiked ? "[&>path]:stroke-transparent" : ""}
                                    fill={isTrackLiked ? "currentColor" : "none"}
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
                                value={currentVolume}
                                onChange={(value) => {
                                    if (typeof value === 'number') {
                                        handleVolumeChange(value);
                                    }
                                }}
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