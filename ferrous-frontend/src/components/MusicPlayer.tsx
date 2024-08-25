"use client";

import React, {FC} from "react";
import {Card, CardBody} from "@nextui-org/card";
import {Button} from "@nextui-org/button";
import {Slider} from "@nextui-org/slider";
import {useMusicPlayerContext} from "@/contexts/MusicPlayerContext";
import {FaRegHeart} from "react-icons/fa";
import {FaCirclePause, FaCirclePlay, FaRepeat, FaShuffle} from "react-icons/fa6";
import {ImNext, ImPrevious} from "react-icons/im";


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
        handlePrevious,
        lowShelfGain,
        highShelfGain,
        lowShelfFreq,
        highShelfFreq,
        handleLowShelfGainChange,
        handleHighShelfGainChange,
        handleLowShelfFreqChange,
        handleHighShelfFreqChange,
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
                                <FaRegHeart
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
                                <FaShuffle size={24} className={isShuffleOn ? "text-primary" : ""}/>
                            </Button>
                            <Button
                                isIconOnly
                                className="data-[hover]:bg-foreground/10"
                                radius="full"
                                variant="light"
                                onClick={handlePrevious}
                            >
                                <ImPrevious size={24}/>
                            </Button>
                            <Button
                                isIconOnly
                                className="data-[hover]:bg-foreground/10"
                                radius="full"
                                variant="light"
                                onClick={handlePlayPause}
                            >
                                {isMusicPlaying ? <FaCirclePause size={54}/> :
                                    <FaCirclePlay size={54}/>}
                            </Button>
                            <Button
                                isIconOnly
                                className="data-[hover]:bg-foreground/10"
                                radius="full"
                                variant="light"
                                onClick={handleNext}
                            >
                                <ImNext size={24}/>
                            </Button>
                            <Button
                                isIconOnly
                                className="data-[hover]:bg-foreground/10"
                                radius="full"
                                variant="light"
                                onClick={toggleRepeat}
                            >
                                <FaRepeat size={24} className={isRepeatOn ? "text-primary" : ""}/>
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

                        <div className="flex items-center mt-4 w-full">
                            <Slider
                                label="Low Shelf Gain"
                                size="md"
                                color="primary"
                                step={1}
                                maxValue={40}
                                minValue={-40}
                                value={lowShelfGain}
                                onChange={(value) => {
                                    if (typeof value === 'number') {
                                        handleLowShelfGainChange(value);
                                    }
                                }}
                                className="w-full max-w-md mx-auto"
                            />
                            <Slider
                                label="High Shelf Gain"
                                size="md"
                                color="secondary"
                                step={1}
                                maxValue={40}
                                minValue={-40}
                                value={highShelfGain}
                                onChange={(value) => {
                                    if (typeof value === 'number') {
                                        handleHighShelfGainChange(value);
                                    }
                                }}
                                className="w-full max-w-md mx-auto ml-4"
                            />
                        </div>

                        <div className="flex items-center mt-4 w-full">
                            <Slider
                                label="Low Shelf Frequency"
                                size="md"
                                color="success"
                                step={10}
                                maxValue={1000}
                                minValue={20}
                                value={lowShelfFreq}
                                onChange={(value) => {
                                    if (typeof value === 'number') {
                                        handleLowShelfFreqChange(value);
                                    }
                                }}
                                className="w-full max-w-md mx-auto"
                            />
                            <Slider
                                label="High Shelf Frequency"
                                size="md"
                                color="danger"
                                step={100}
                                maxValue={20000}
                                minValue={2000}
                                value={highShelfFreq}
                                onChange={(value) => {
                                    if (typeof value === 'number') {
                                        handleHighShelfFreqChange(value);
                                    }
                                }}
                                className="w-full max-w-md mx-auto ml-4"
                            />
                        </div>

                    </div>
                </div>
            </CardBody>
        </Card>
    );
};

export default MusicPlayer;