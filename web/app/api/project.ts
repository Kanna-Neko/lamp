import { BaseResponse } from "./api"

export async function read_all_project() {
    const res = await fetch(`${process.env.API_URL}/api/admin/project_all`)
    if (!res.ok) {
        throw Error("Failed to fetch project data")
    }
    const data:ReadAllProjectResponse = await res.json()
    if (data.code != 200) {
        throw Error(data.message)
    }
    return data.data
}

export interface ReadAllProjectResponse extends BaseResponse{
    data: {
        project: {
            _id: string,
            name: string,
            description: string,
            root_folder_id: string
        }[]
    }
}

export async function create_project() {
    const res = await fetch(`${process.env.API_URL}/api/admin/project_all`)
    if (!res.ok) {
        throw Error("Failed to fetch project data")
    }
    const data:CreateProjectResponse = await res.json()
    if (data.code != 200) {
        throw Error(data.message)
    }
    return data.data
}

export interface CreateProjectResponse extends BaseResponse{
    data: Data;
}

export interface Data {
    project: Project;
    script_folder: ScriptFolder;
}

export interface Project {
    _id: string;
    description: string;
    name: string;
    root_folder_id: string;
}

export interface ScriptFolder {
    _id: string;
    name: string;
}