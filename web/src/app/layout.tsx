import type { Metadata } from "next"
import { Geist, Geist_Mono } from "next/font/google"
import "./globals.css"
import { Analytics } from "@vercel/analytics/next"
import { Viewport } from "next"

const geistSans = Geist({
  variable: "--font-geist-sans",
  subsets: ["latin"],
})

const geistMono = Geist_Mono({
  variable: "--font-geist-mono",
  subsets: ["latin"],
})

export const metadata: Metadata = {
  title: "MPL - Motion Programming Language | AI-Powered 3D Animation",
  description:
    "Revolutionary semantic language for 3D motion and animation. Transform natural language into 3D movements with AI-friendly syntax. Perfect for LLM training, motion synthesis, and creative animation workflows.",
  keywords: [
    "motion programming language",
    "3D animation",
    "AI motion synthesis",
    "LLM training",
    "text to motion",
    "semantic animation",
    "motion generation",
    "3D pose programming",
    "MMD",
    "MikuMikuDance",
    "large motion models",
    "procedural animation",
    "motion capture",
    "animation AI",
    "natural language animation",
    "cross-modal AI",
    "motion language model",
  ],
  authors: [{ name: "MPL Team" }],
  creator: "MPL Team",
  publisher: "MPL",
  category: "Technology",
  openGraph: {
    title: "MPL - Motion Programming Language | AI-Powered 3D Animation",
    description:
      "Revolutionary semantic language for 3D motion and animation. Transform natural language into 3D movements with AI-friendly syntax. Perfect for LLM training and motion synthesis.",
    url: "https://mmd-mpl.vercel.app",
    siteName: "MPL - Motion Programming Language",
    type: "website",
    locale: "en_US",
  },
  robots: {
    index: true,
    follow: true,
    googleBot: {
      index: true,
      follow: true,
      "max-video-preview": -1,
      "max-image-preview": "large",
      "max-snippet": -1,
    },
  },
  alternates: {
    canonical: "https://mpl.love",
  },
}

export const viewport: Viewport = {
  width: "device-width",
  initialScale: 1,
  maximumScale: 1,
  userScalable: false,
}

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode
}>) {
  return (
    <html lang="en" className="select-none outline-none">
      <body className={`${geistSans.variable} ${geistMono.variable} antialiased`}>{children}</body>
      <Analytics />
    </html>
  )
}
