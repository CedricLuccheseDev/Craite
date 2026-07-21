// The product name carries its own joke: the "ai" sits inside "craite".
// Stamping it in the accent is the whole logo, no image needed.
export function Wordmark({ className = '' }: { className?: string }) {
  return (
    <span className={`font-display font-bold tracking-tight ${className}`}>
      cr<span className="text-accent">ai</span>te
    </span>
  )
}
