import {useCallback, useState} from "react";
import {Howl} from "howler";

export const useMusicPlayer = () => {
    const [playlist, setPlaylist] = useState<string[]>([]);
    const [currentTrackName, setCurrentTrackName] = useState<string>("");
    const [currentTrackIdx, setCurrentTrackIdx] = useState<number>(0);
    const [isMusicPlaying, setIsMusicPlaying] = useState<boolean>(false);
    const [isShuffleOn, setIsShuffleOn] = useState<boolean>(false);
    const [isRepeatOn, setIsRepeatOn] = useState<boolean>(false);
    const [isTrackLiked, setIsTrackLiked] = useState(false);
    const [currentVolume, setCurrentVolume] = useState<number>(0.5);
    const [lowShelfGain, setLowShelfGain] = useState<number>(0);
    const [highShelfGain, setHighShelfGain] = useState<number>(0);
    const [lowShelfFreq, setLowShelfFreq] = useState<number>(200);
    const [highShelfFreq, setHighShelfFreq] = useState<number>(2000);
    const [lowshelfFilter, setLowshelfFilter] = useState<BiquadFilterNode | null>(null);
    const [highshelfFilter, setHighshelfFilter] = useState<BiquadFilterNode | null>(null);
    const [howl, setHowl] = useState<Howl | null>(null);

    const handleUpdatePlaylistContents = useCallback((newPlaylistContents: string[]) => {
        console.debug("Updating playlist contents");
        setPlaylist(newPlaylistContents);
    }, []);

    const handlePlay = useCallback((fileName: string) => {
        console.debug(`Starting playback of a new track: ${fileName}`);
        if (howl) {
            howl.stop();
        }

        const newHowl = new Howl({
            src: [`http://localhost:13337/library/play/${encodeURIComponent(fileName)}`],
            // html5: true,
            html5: false, // TODO: Enable HTML5 audio when supported, might now work currently with bass / treble filters
            volume: currentVolume,
            onload: () => {
                const audioCtx = Howler.ctx;
                const source = Howler.masterGain;
                const newLowshelfFilter = audioCtx.createBiquadFilter();
                const newHighshelfFilter = audioCtx.createBiquadFilter();

                newLowshelfFilter.type = 'lowshelf';
                newLowshelfFilter.frequency.value = lowShelfFreq;
                newLowshelfFilter.gain.value = lowShelfGain;

                console.debug(`Lowshelf filter freq: ${newLowshelfFilter.frequency.value}, gain: ${newLowshelfFilter.gain.value}`);

                newHighshelfFilter.type = 'highshelf';
                newHighshelfFilter.frequency.value = highShelfFreq;
                newHighshelfFilter.gain.value = highShelfGain;

                console.debug(`Highshelf filter freq: ${newHighshelfFilter.frequency.value}, gain: ${newHighshelfFilter.gain.value}`);

                source.disconnect();
                source.connect(newLowshelfFilter);
                newLowshelfFilter.connect(newHighshelfFilter);
                newHighshelfFilter.connect(audioCtx.destination);

                setLowshelfFilter(newLowshelfFilter);
                setHighshelfFilter(newHighshelfFilter);
            },
            onplay: () => {
                setIsMusicPlaying(true);
                setCurrentTrackName(fileName);
            },
            onend: () => {
                setIsMusicPlaying(false);
                if (isRepeatOn) {
                    // TODO: Doesn't work correctly
                    console.debug(`Repeat is on, playing the same track again`);
                    newHowl.play();
                } else {
                    const nextIndex = (currentTrackIdx + 1) % playlist.length;
                    setCurrentTrackIdx(nextIndex);
                    // TODO: stack overflow here?
                    console.debug(`Repeat is off, playing next track: ${playlist[nextIndex]}`);
                    handlePlay(playlist[nextIndex]);
                }
            },
            onpause: () => {
                setIsMusicPlaying(false);
            },
            onstop: () => {
                setIsMusicPlaying(false);
                setCurrentTrackName("");
            },
            onloaderror: (id, error) => {
                console.error(`Error loading track with id ${id}: ${error}`);
                setIsMusicPlaying(false);
                setCurrentTrackName("");
            },
            onplayerror: (id, error) => {
                console.error(`Error playing track with id ${id}: ${error}`);
                setIsMusicPlaying(false);
                setCurrentTrackName("");
            },
        })

        setHowl(newHowl);
        const index = playlist.indexOf(fileName);
        setCurrentTrackIdx(index);
        setCurrentTrackName(fileName);
        newHowl.play();

    }, [howl, currentVolume, currentTrackIdx, isRepeatOn, playlist, lowShelfGain, highShelfGain, lowShelfFreq, highShelfFreq]);

    const handleNext = useCallback(() => {
        const nextIndex = (currentTrackIdx + 1) % playlist.length;
        setCurrentTrackIdx(nextIndex);
        console.debug(`Playing next track: ${playlist[nextIndex]}`);
        handlePlay(playlist[nextIndex]);
    }, [currentTrackIdx, playlist, handlePlay]);

    const handlePrevious = useCallback(() => {
        const prevIndex = (currentTrackIdx - 1 + playlist.length) % playlist.length;
        setCurrentTrackIdx(prevIndex);
        console.debug(`Playing previous track: ${playlist[prevIndex]}`);
        handlePlay(playlist[prevIndex]);
    }, [currentTrackIdx, playlist, handlePlay]);

    const handlePlayPause = useCallback(() => {
        if (howl) {
            if (isMusicPlaying) {
                console.debug("Pausing playback");
                howl.pause();
            } else {
                console.debug("Resuming playback after pausing");
                howl.play();
            }
            setIsMusicPlaying(!isMusicPlaying);
        }
    }, [howl, isMusicPlaying]);

    const handleSeekTo = useCallback((position: number) => {
        if (howl) {
            howl.seek(position);
        }
    }, [howl]);

    const handleVolumeChange = useCallback((newVolume: number) => {
        console.debug(`Changing volume from ${currentVolume} to ${newVolume}`);
        setCurrentVolume(newVolume);
        if (howl) {
            howl.volume(newVolume);
        }
    }, [howl, currentVolume]);

    const handleLowShelfGainChange = useCallback((value: number) => {
        setLowShelfGain(value);
        if (lowshelfFilter) {
            console.debug(`Updating lowshelf filter gain to ${value}`);
            lowshelfFilter.gain.value = value;
        } else {
            console.warn(`Lowshelf filter not initialized`);
        }
    }, [lowshelfFilter]);

    const handleHighShelfGainChange = useCallback((value: number) => {
        setHighShelfGain(value);
        if (highshelfFilter) {
            console.debug(`Updating highshelf filter gain to ${value}`);
            highshelfFilter.gain.value = value;
        } else {
            console.warn(`Highshelf filter not initialized`);
        }
    }, [highshelfFilter]);

    const handleLowShelfFreqChange = useCallback((value: number) => {
        setLowShelfFreq(value);
        if (lowshelfFilter) {
            console.debug(`Updating lowshelf filter frequency to ${value}`);
            lowshelfFilter.frequency.value = value;
        } else {
            console.warn(`Lowshelf filter not initialized`);
        }
    }, [lowshelfFilter]);

    const handleHighShelfFreqChange = useCallback((value: number) => {
        setHighShelfFreq(value);
        if (highshelfFilter) {
            console.debug(`Updating highshelf frequency to ${value}`);
            highshelfFilter.frequency.value = value;
        } else {
            console.warn(`Highshelf filter not initialized`);
        }
    }, [highshelfFilter]);

    const toggleShuffle = useCallback(() => {
        setIsShuffleOn(prev => {
            const newValue = !prev;
            console.debug(`Toggling shuffle mode to ${newValue}`);
            return newValue;
        });
    }, []);

    const toggleRepeat = useCallback(() => {
        setIsRepeatOn(prev => {
            const newValue = !prev;
            console.debug(`Toggling repeat mode to ${newValue}`);
            return newValue;
        });
    }, []);

    const toggleTrackLiked = useCallback(() => {
        setIsTrackLiked(prev => {
            const newValue = !prev;
            console.debug(`Toggling track liked status to ${newValue}`);
            return newValue;
        });
    }, []);

    return {
        playlist,
        currentTrackName,
        currentTrackIdx,
        currentVolume,
        isMusicPlaying,
        isShuffleOn,
        isRepeatOn,
        isTrackLiked,
        lowShelfGain,
        highShelfGain,
        lowShelfFreq,
        highShelfFreq,
        toggleShuffle,
        toggleRepeat,
        toggleTrackLiked,
        handleUpdatePlaylistContents,
        handleVolumeChange,
        handlePlay,
        handlePlayPause,
        handleNext,
        handlePrevious,
        handleSeekTo,
        handleLowShelfGainChange,
        handleHighShelfGainChange,
        handleLowShelfFreqChange,
        handleHighShelfFreqChange,
    }
}