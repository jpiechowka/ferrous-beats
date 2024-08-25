import {MusicPlayerProvider} from '@/contexts/MusicPlayerContext'
import {NextUIProvider} from '@nextui-org/react'
import {ToolsProvider} from "@/contexts/ToolsContext";
import {Toaster} from 'sonner';

export function Providers({children}: { children: React.ReactNode }) {
    return (
        <NextUIProvider>
            <ToolsProvider>
                <MusicPlayerProvider>
                    {children}
                    <Toaster toastOptions={{
                        unstyled: false,
                        classNames: {
                            error: 'bg-red-600 text-slate-100',
                            success: 'text-green-400',
                            warning: 'text-yellow-400',
                            info: 'bg-blue-400',
                        },
                    }}/>
                </MusicPlayerProvider>
            </ToolsProvider>
        </NextUIProvider>
    )
}
