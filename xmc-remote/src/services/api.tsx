export enum Power {
    On = "On",
    Off = "Off",
}

export type XmcStatus = {
    power: Power;
    source: string;
    volume: number;
    mode: string;
};

export const getStatus = () => fetch("/info").then(res => res.json()).then(r => r as XmcStatus)
export const volumeUp = () => fetch("/volume/up", { method: "POST" })
export const volumeDown = () => fetch("/volume/down", { method: "POST" })
export const setVolume = (volume: number) => fetch(`/volume/${volume}`, { method: "POST" })
export const setMode = (mode: string) => fetch(`mode/${mode}`, { method: "POST" })
export const setSource = (sourceIndex: number) => fetch(`hdmi/${sourceIndex}`, { method: "POST" })
export const powerOn = () => fetch(`power/on`, { method: "POST" })
export const standBy = () => fetch(`power/off`, { method: "POST" })