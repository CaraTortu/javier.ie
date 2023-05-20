"use client"

import { motion } from "framer-motion";
import Link from "next/link";
import { FC, ReactNode } from "react";

interface MenuItemProps {
  children: ReactNode,
  href: string
}

const variants = {
  open: {
    y: 0,
    opacity: 1,
    transition: {
      y: { stiffness: 1000, velocity: -100 }
    }
  },
  closed: {
    y: 50,
    opacity: 0,
    transition: {
      y: { stiffness: 1000 }
    }
  }
};

export const MenuItem: FC<MenuItemProps> = ({ children, href }) => {
  return (
    <motion.div
      variants={variants}
      whileHover={{ scale: 1.05, color: "#000", fontWeight: "500" }}
      whileTap={{ scale: 0.95 }}
      className="list-none mb-5 flex items-center cursor-pointer text-gray-600"
    >
      <Link href={href}>{children}</Link>
    </motion.div>
  );
};
