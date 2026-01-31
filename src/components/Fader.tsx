
import * as React from \"react\"
import * as SliderPrimitive from \"@radix-ui/react-slider\"
import { clsx, type ClassValue } from \"clsx\"
import { twMerge } from \"tailwind-merge\"

function cn(...inputs: ClassValue[]) {
    return twMerge(clsx(inputs))
}

const Slider = React.forwardRef<
    React.ElementRef<typeof SliderPrimitive.Root>,
    React.ComponentPropsWithoutRef<typeof SliderPrimitive.Root>
>(({ className, ...props }, ref) => (
    <SliderPrimitive.Root
        ref={ref}
        className={cn(
            \"relative flex w-full touch-none select-none items-center\",
            className
        )}
        {...props}
    >
        <SliderPrimitive.Track className=\"relative h-full w-full grow overflow-hidden rounded-full bg-secondary/50\">
            <SliderPrimitive.Range className=\"absolute w-full bg-primary\" />
        </SliderPrimitive.Track>
        <SliderPrimitive.Thumb className=\"block h-5 w-5 rounded-full border-2 border-primary bg-background ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50\" />
    </SliderPrimitive.Root>
))
Slider.displayName = SliderPrimitive.Root.displayName

interface FaderProps {
    label: string;
    value: number; // dB -128 to 0
    onChange: (val: number) => void;
    color?: string;
}

export const Fader: React.FC<FaderProps> = ({ label, value, onChange, color }) => {
    // Map dB (-128 to 0) to percentage (0 to 100) or just use the range directly
    // Let's use range 0-128 internally for the slider to keep it positive, then map back

    // Actually Radix slider handles ranges fine.
    // min = -128, max = 0

    return (
        <div className=\"flex flex-col items-center gap-4 h-full\">
            <div className=\"h-[200px] flex items-center justify-center\">
                <SliderPrimitive.Root
                    className=\"relative flex h-full w-5 touch-none select-none flex-col items-center\"
                    orientation=\"vertical\"
                    min={-128}
                    max={0}
                    step={1}
                    value={[value]}
                    onValueChange={(vals) => onChange(vals[0])}
                >
                    <SliderPrimitive.Track className=\"relative h-full w-2 shrink-0 grow rounded-full bg-base-300 overflow-hidden border border-white/5\">
                        <SliderPrimitive.Range className={cn(\"absolute w-full rounded-full bg-primary\", color)} />
                    </SliderPrimitive.Track>
                    <SliderPrimitive.Thumb
                        className={cn(\"block h-6 w-6 rounded-full border-2 border-white/20 bg-white shadow-lg ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 hover:scale-110 transition-transform cursor-grab active:cursor-grabbing\", color?.replace(\"bg-\", \"border-\"))}
                        aria-label=\"Volume\"
                    />
                </SliderPrimitive.Root>
            </div>

            <div className=\"flex flex-col items-center gap-1\">
                <span className=\"font-mono text-xs text-primary\">
                    {value <= -128 ? \"-inf\" : `${value}dB`}
                </span>
                <span className=\"font-bold text-sm tracking-wide text-base-content/80\">
                    {label}
                </span>
            </div>
        </div>
    );
};
