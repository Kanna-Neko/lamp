'use client'
import Link from "next/link"
import { usePathname } from "next/navigation"
import Image from 'next/image'
import { motion } from "framer-motion"

export default function Layout({
    children
}: {
    children:React.ReactNode,
}) {
    const path_name = usePathname()
    const directory: directory[] = [
        {name: "introduce", url: "",icon:<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" strokeLinecap="round" strokeLinejoin="round"><path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"></path><polyline points="9 22 9 12 15 12 15 22"></polyline></svg>},
        {name: "project", url: "/project", icon:<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" strokeLinecap="round" strokeLinejoin="round"><path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"></path></svg>},
        {name: "user", url: "/user",icon: <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" strokeLinecap="round" strokeLinejoin="round"><path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"></path><circle cx="12" cy="7" r="4"></circle></svg>}
    ]
    return (
    <div className="flex flex-col h-screen">
        <div className="h-12 border-black/20 border-b shadow-sm flex-none flex items-center p-4">
            <Link href={"/dashboard"}>
                <Image src="/favicon.ico"
                    width={36}
                    height={36}
                    alt="Picture of the author"
                />
            </Link>
        </div>
        <div className="flex flex-auto overflow-auto">
            <div className="w-min-36 border-r border-black/20 box-border p-4 flex flex-col gap-4 ">
                {directory.map((item,index) => {
                    return <Link href={`/dashboard/${item.url}`} key={index.toString()}>
                        <motion.div  initial={{y:500, opacity:0}} animate={{y:0, opacity:1,transition:{duration:0.3+index/10}}} whileTap={{scale:0.95}} whileHover={{rotate:-6,transition:{duration:0.02}}} className={"border border-black/20 align-middle rounded-md shadow-md py-2 px-3  active:scale-90 duration-100 hover:-rotate-6 flex gap-3 items-center font-medium stroke-2 " + (path_name == "/dashboard" + item.url ? "bg-gray-900 text-gray-100 stroke-white":"text-gray-800 stroke-gray-800 hover:bg-black/10")}>
                            <div className="w-4">
                                {item.icon}
                            </div>
                            {item.name}
                        </motion.div>
                    </Link>
                })}
            </div>
            <div className="flex-auto">{children}</div>
        </div>
    </div>)
}

interface directory {
    name: string,
    url: string,
    icon?: React.ReactNode
}