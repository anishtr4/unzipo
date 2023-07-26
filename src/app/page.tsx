import Image from 'next/image'
import { Button } from "@/components/ui/button"
import { Mail } from "lucide-react"

export default function Home() {
  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-24">
 <Button>
      <Mail className="mr-2 h-4 w-4" /> Login with Email
    </Button>
    </main>
  )
}
