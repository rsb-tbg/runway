import { invoke } from '@tauri-apps/api/tauri'

export function set_current_view(new_view:string) {
    invoke('set_current_view', {view: new_view})
}

export function next_view() {
    invoke('next_view')
}

export function prev_view() {
    invoke('prev_view')
}

export type Param = {
    name: string,
    value: number,
    min: number,
    max: number,
    increment: string,
}

export type Threshold = {
    name: string,
    value: number,
    min: number,
    max: number,
    increment: string,
    include_results: string,
}