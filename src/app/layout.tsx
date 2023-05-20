import { NavBar } from '@/components/Navbar'
import './globals.css'
import { Roboto_Mono } from 'next/font/google'

const roboto = Roboto_Mono({ subsets: ['latin'] })

export const metadata = {
  title: 'Javier\'s portfolio'
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en">
      <body className={roboto.className}>
        <NavBar />
          {children}
        </body>
    </html>
  )
}
