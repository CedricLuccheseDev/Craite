import { createFileRoute } from '@tanstack/react-router'
import { SiteHeader } from '~/components/SiteHeader'
import { SiteFooter } from '~/components/SiteFooter'
import { Hero } from '~/components/home/Hero'
import { HowItWorks } from '~/components/home/HowItWorks'
import { Trust } from '~/components/home/Trust'
import { Daws } from '~/components/home/Daws'
import { Faq } from '~/components/home/Faq'
import { Download } from '~/components/home/Download'

export const Route = createFileRoute('/')({
  component: Home,
})

function Home() {
  return (
    <>
      <SiteHeader />
      <main>
        <Hero />
        <HowItWorks />
        <Trust />
        <Daws />
        <Faq />
        <Download />
      </main>
      <SiteFooter />
    </>
  )
}
