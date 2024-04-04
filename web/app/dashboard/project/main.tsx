'use client'

import ReactModal from 'react-modal'
import { ReadAllProjectResponse } from "@/app/api/project"
import {AnimationProps, motion} from 'framer-motion'
import { useRef, useState } from "react"
import { SubmitHandler, useForm } from 'react-hook-form'
type projectType =  ReadAllProjectResponse['data']['project']

interface CreateProjectForm {
    name: string
}

export default function Project({project}: {project:projectType}) {
    const [is_add_hover,set_is_add_hover] = useState(false)
    const dialog = useRef<HTMLDialogElement>(null)
    const {
        register,
        handleSubmit,
        formState:{errors,isSubmitted,isSubmitting},
        reset
    } = useForm<CreateProjectForm>()
    const add_hover_animate: AnimationProps['variants'] = {
        "hover": {
            paddingLeft: "2px",
            width: 'auto'
        },
        "no-hover": {
            paddingLeft: 0,
            width: 0
        }
    }

    const open_modal = () => {
        reset()
        dialog.current?.showModal()
    }

    const create_project: SubmitHandler<CreateProjectForm> = async (data) => {
    }
    return <>
        <motion.div className="m-2 p-2 h-96 border rounded-md shadow-md" initial={{skewX:-10, opacity:0, y:50}} animate={{skewX:0, opacity:1, y:0}} transition={{duration:0.5}}>
            <div className="flex justify-end">
                <motion.button onClick={open_modal} className="rounded-md border text-sm p-1 px-2 shadow-md flex items-center hover:bg-black/10 duration-300" onMouseEnter={() => {set_is_add_hover(true)}} onMouseLeave={()=>{set_is_add_hover(false)}}>
                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="24" viewBox="0 0 24 24" fill="none" stroke="#000000" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><line x1="12" y1="5" x2="12" y2="19"></line><line x1="5" y1="12" x2="19" y2="12"></line></svg>
                    <motion.div className="overflow-hidden text-nowrap" initial={{width:0}} animate={is_add_hover ? 'hover' : 'no-hover'} variants={add_hover_animate}>New project</motion.div>
                </motion.button>
            </div>
        </motion.div>
        <dialog ref={dialog} className="modal">
            <div className="modal-box">
            <form method="dialog">
                <button className="btn btn-sm btn-circle btn-ghost absolute right-2 top-2">✕</button>
            </form>
                <h2 className="text-lg font-medium">New project</h2>
                <form onSubmit={handleSubmit(create_project)}>
                    <label className="form-control w-full max-w-xs">
                        <div className="label">
                            <span className="label-text">What is project name?</span>
                        </div>
                        <input type="text" placeholder="Type here" className={"input input-bordered w-full max-w-xs " + (errors.name && isSubmitted ? " input-error" : "")} {...register("name", {required:true})} />
                        <div className="label">
                            <span className={"label-text-alt" + (errors.name && isSubmitted ? " text-red-600" : " invisible")}>Please input name</span>
                        </div>
                    </label>
                    <div className='flex justify-end mt-0'>
                        {
                            isSubmitting ? 
                            <span className="loading loading-spinner loading-md"></span>
                            :
                            <button className='btn'>create</button>
                        }
                    </div>
                </form>
            </div>
            <form method="dialog" className="modal-backdrop">
                <button>Close</button>
            </form>
        </dialog>
    </>
}