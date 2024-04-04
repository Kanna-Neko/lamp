import { read_all_project, ReadAllProjectResponse } from "@/app/api/project"
import Project from './main'
export default async function Page() {
    const res = await read_all_project()
    return <>
        <Project project={res.project}/>
    </>
}