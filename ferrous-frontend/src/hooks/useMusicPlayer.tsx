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
            html5: true,
            volume: currentVolume,
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

    }, [howl, currentVolume, currentTrackIdx, isRepeatOn, playlist]);

    const handleNext = useCallback(() => {
        const nextIndex = (currentTrackIdx + 1) % playlist.length;
        if (nextIndex !== currentTrackIdx) {
            setCurrentTrackIdx(nextIndex);
            console.debug(`Playing next track: ${playlist[nextIndex]}`);
            handlePlay(playlist[nextIndex]);
        }
    }, [currentTrackIdx, playlist, handlePlay]);

    const handlePrevious = useCallback(() => {
        const prevIndex = (currentTrackIdx - 1 + playlist.length) % playlist.length;
        if (prevIndex !== currentTrackIdx) {
            setCurrentTrackIdx(prevIndex);
            console.debug(`Playing previous track: ${playlist[prevIndex]}`);
            handlePlay(playlist[prevIndex]);
        }
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

    const toggleShuffle = useCallback(() => {
        setIsShuffleOn(prev => !prev);
    }, []);

    const toggleRepeat = useCallback(() => {
        setIsRepeatOn(prev => !prev);
    }, []);

    const toggleTrackLiked = useCallback(() => {
        setIsTrackLiked(prev => !prev);
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
    }
}