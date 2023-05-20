"use client"

import { motion, useCycle } from "framer-motion";
import { MenuToggle } from "./MenuToggle";
import { MenuItem } from "./MenuItem";


const sidebarOptions = {
    open: {
        clipPath: `circle(2200px at 0px 0px)`,
        transition: {
            type: "spring",
            stiffness: 20
        }
    },
    closed: {
        clipPath: "circle(100px at 0px 0px)",
        transition: {
            delay: 0.2,
            type: "spring",
            stiffness: 450,
            damping: 35
        }
    }
};

const ULOtions = {
    open: {
        transition: { staggerChildren: 0.07, delayChildren: 0.2 }
    },
    closed: {
        transition: { staggerChildren: 0.05, staggerDirection: -1 }
    }
};

export const NavBar = () => {
    const [isOpen, toggleOpen] = useCycle(false, true);

    return (
        <motion.nav
            initial={false}
            animate={isOpen ? "open" : "closed"}
            className="absolute top-0 left-0 bottom-0 w-72"
        >
            <motion.div className="absolute top-0 left-0 bottom-0 w-72 bg-white" variants={sidebarOptions} />

            <motion.ul variants={ULOtions} className="m-0 p-6 absolute top-24 w-56" onClick={() => toggleOpen()}>
                <MenuItem href="/">Home</MenuItem>
                <MenuItem href="/skills">Skills</MenuItem> 
                <MenuItem href="/studies">Studies</MenuItem>
                <MenuItem href="/contact">Contact</MenuItem>
            </motion.ul>

            <MenuToggle toggle={() => toggleOpen()} />
        </motion.nav>
    );
};