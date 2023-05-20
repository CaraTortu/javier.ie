"use client"

import { motion } from "framer-motion";
import { FC } from "react";

interface pathProps {
  d?: string,
  transition?: { duration: number},
  variants: {
    open: {
      opacity?: number,
      d?: string
    },
    closed: {
      opacity?: number,
      d?: string
    }
  }
}

interface MenuToggleProps {
  toggle: React.MouseEventHandler
}

const Path: FC<pathProps> = props => (
  <motion.path
    fill="transparent"
    strokeWidth="3"
    stroke="hsl(0, 0%, 18%)"
    strokeLinecap="round"
    {...props}
  />
);

export const MenuToggle: FC<MenuToggleProps> = ({ toggle }) => (
  <button onClick={toggle} className="outline-none border-none cursor-pointer absolute top-4 left-4 w-12 h-12 bg-transparent flex items-center justify-center">
    <svg width="23" height="23" viewBox="0 0 23 20">
      <Path
        variants={{
          closed: { d: "M 2 2.5 L 20 2.5" },
          open: { d: "M 3 16.5 L 17 2.5" }
        }}
      />
      <Path
        d="M 2 9.423 L 20 9.423"
        variants={{
          closed: { opacity: 1 },
          open: { opacity: 0 }
        }}
        transition={{ duration: 0.1 }}
      />
      <Path
        variants={{
          closed: { d: "M 2 16.346 L 20 16.346" },
          open: { d: "M 3 2.5 L 17 16.346" }
        }}
      />
    </svg>
  </button>
);
