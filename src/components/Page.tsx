"use client"

import { motion } from "framer-motion";
import { Children, FC, ReactNode } from "react";

interface PageProps {
    children: ReactNode
}

const ContentVariants = {
  initial: { y: 50, opacity: 0 },
  animate: {
    y: 0,
    opacity: 1,
    transition: { staggerChildren: 0.05, ease: "easeOut", duration: 0.3 },
  },
}

export const Page: FC<PageProps> = ({ children }) => {
  return (
    <motion.div variants={ContentVariants} initial="initial" animate="animate">
      {Children.map(children, child => (
        <motion.div variants={ContentVariants}>
          {child}
        </motion.div>
      ))}
    </motion.div>
  )
}
