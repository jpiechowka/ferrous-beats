"use client";

import React, {createContext, FC, ReactNode, useContext} from 'react';
import {useMusicPlayer} from "@/hooks/useMusicPlayer";

const MusicPlayerContext = createContext<ReturnType<typeof useMusicPlayer> | undefined>(undefined);

export const MusicPlayerProvider: FC<{ children: ReactNode }> = ({children}) => {
    const musicPlayerState = useMusicPlayer();

    return (
        <MusicPlayerContext.Provider value={musicPlayerState}>
            {children}
        </MusicPlayerContext.Provider>
    );
};

export const useMusicPlayerContext = () => {
    const context = useContext(MusicPlayerContext);
    if (context === undefined) {
        throw new Error('useMusicPlayerContext must be used within a MusicPlayerProvider');
    }
    return context;
};