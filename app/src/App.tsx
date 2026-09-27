import { useEffect, useRef, useState } from 'react';
import { 
  Shield, Scan, CheckCircle, AlertTriangle, 
  Clock, Lock, Volume2, VolumeX, 
  Download, ChevronDown, Cpu, Zap, Eye,
  FileText, ArrowRight,
  BarChart3
} from 'lucide-react';
import { Button } from '@/components/ui/button';
import { Slider } from '@/components/ui/slider';
import { Badge } from '@/components/ui/badge';
import { Progress } from '@/components/ui/progress';
import './App.css';

// Types
interface RiskCategory {
  name: string;
  percentage: number;
  color: string;
  description: string;
}

interface FlaggedSegment {
  id: string;
  start: string;
  end: string;
  severity: 'High' | 'Medium' | 'Low';
  issue: string;
}

interface RemediationOption {
  id: string;
  title: string;
  impact: 'Low' | 'Medium' | 'High';
  description: string;
  selected: boolean;
}

// Data
const riskCategories: RiskCategory[] = [
  { name: 'Copyright', percentage: 45, color: '#00F0FF', description: 'Audio fingerprint matches rights-managed track' },
  { name: 'Misinformation', percentage: 25, color: '#7B61FF', description: 'Claims require additional context' },
  { name: 'Hate Speech', percentage: 20, color: '#FF4ECD', description: 'Language flagged for review' },
  { name: 'Violence', percentage: 10, color: '#FFD166', description: 'Graphic imagery detected' },
];

const flaggedSegments: FlaggedSegment[] = [
  { id: '1', start: '00:34', end: '00:48', severity: 'High', issue: 'Copyrighted audio detected' },
  { id: '2', start: '01:12', end: '01:19', severity: 'Medium', issue: 'Potentially sensitive imagery' },
  { id: '3', start: '02:05', end: '02:11', severity: 'Low', issue: 'Borderline language' },
];

const remediationOptions: RemediationOption[] = [
  { id: '1', title: 'Replace audio track', impact: 'Low', description: 'Swap with royalty-free alternative', selected: true },
  { id: '2', title: 'Blur segment + add context', impact: 'Medium', description: 'Add explanatory overlay', selected: false },
  { id: '3', title: 'Remove segment', impact: 'High', description: 'Delete 14 seconds of content', selected: false },
];

// Components
const Navigation = () => (
  <nav className="fixed top-0 left-0 right-0 z-50 flex items-center justify-between px-8 py-5">
    <div className="flex items-center gap-2">
      <Shield className="w-7 h-7 text-[#00F0FF]" />
      <span className="text-xl font-semibold tracking-tight">ContentGuard</span>
    </div>
    <div className="hidden md:flex items-center gap-8">
      <a href="#product" className="text-sm text-[#A7B0C8] hover:text-white transition-colors">Product</a>
      <a href="#demo" className="text-sm text-[#A7B0C8] hover:text-white transition-colors">Demo</a>
      <a href="#pricing" className="text-sm text-[#A7B0C8] hover:text-white transition-colors">Pricing</a>
      <a href="#docs" className="text-sm text-[#A7B0C8] hover:text-white transition-colors">Docs</a>
    </div>
    <Button variant="outline" className="border-[#00F0FF]/50 text-[#00F0FF] hover:bg-[#00F0FF]/10 rounded-full px-5">
      Start Free Scan
    </Button>
  </nav>
);

const VideoCard = ({ 
  children, 
  className = '',
  showScanLine = false,
  showDivider = false,
  image = '/video_thumb_cyberpunk.jpg'
}: { 
  children?: React.ReactNode; 
  className?: string;
  showScanLine?: boolean;
  showDivider?: boolean;
  image?: string;
}) => (
  <div className={`video-card ${className}`}>
    <img 
      src={image} 
      alt="Video preview" 
      className="w-full h-full object-cover"
    />
    {showScanLine && <div className="scan-line" />}
    {showDivider && (
      <div className="absolute inset-y-0 left-1/2 w-px bg-[#00F0FF] shadow-[0_0_20px_rgba(0,240,255,0.5)]" />
    )}
    {children}
  </div>
);



const Section1Hero = () => {
  const sectionRef = useRef<HTMLDivElement>(null);
  const [isVisible, setIsVisible] = useState(false);

  useEffect(() => {
    const observer = new IntersectionObserver(
      ([entry]) => {
        if (entry.isIntersecting) {
          setIsVisible(true);
        }
      },
      { threshold: 0.2 }
    );
    if (sectionRef.current) observer.observe(sectionRef.current);
    return () => observer.disconnect();
  }, []);

  return (
    <section ref={sectionRef} className="section-pinned flex items-center justify-center">
      <div className={`transition-all duration-1000 ${isVisible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-10'}`}>
        <VideoCard showScanLine>
          <div className="absolute inset-0 bg-gradient-to-r from-black/70 via-transparent to-transparent" />
          <div className="absolute left-8 top-1/2 -translate-y-1/2 max-w-md">
            <h1 className="text-5xl md:text-6xl font-bold mb-4 leading-tight">
              Upload. Scan.<br />
              <span className="text-gradient">Publish with Confidence.</span>
            </h1>
            <p className="text-[#A7B0C8] text-lg mb-6">
              Detect policy risks before you go live. Frame-accurate. Private. Built for creators.
            </p>
            <Button className="bg-[#00F0FF] text-black hover:bg-[#00F0FF]/90 rounded-full px-8 py-6 text-base font-medium">
              Start Free Scan
              <ArrowRight className="w-5 h-5 ml-2" />
            </Button>
          </div>
        </VideoCard>
      </div>

      <div className={`analytics-panel transition-all duration-1000 delay-300 ${isVisible ? 'opacity-100 translate-x-0' : 'opacity-0 translate-x-20'}`}>
        <h3 className="font-mono text-xs uppercase tracking-wider text-[#A7B0C8] mb-4">Content Analysis</h3>
        <div className="space-y-4">
          <div className="flex items-center gap-3">
            <div className="w-8 h-8 rounded-lg bg-[#00F0FF]/10 flex items-center justify-center">
              <Scan className="w-4 h-4 text-[#00F0FF]" />
            </div>
            <div>
              <p className="text-sm font-medium">Scan Status</p>
              <p className="text-xs text-[#00E5A0]">Ready</p>
            </div>
          </div>
          <div className="flex items-center gap-3">
            <div className="w-8 h-8 rounded-lg bg-[#7B61FF]/10 flex items-center justify-center">
              <Clock className="w-4 h-4 text-[#7B61FF]" />
            </div>
            <div>
              <p className="text-sm font-medium">Estimated Time</p>
              <p className="text-xs text-[#A7B0C8]">~2 min</p>
            </div>
          </div>
          <div className="flex items-center gap-3">
            <div className="w-8 h-8 rounded-lg bg-[#FF4ECD]/10 flex items-center justify-center">
              <Lock className="w-4 h-4 text-[#FF4ECD]" />
            </div>
            <div>
              <p className="text-sm font-medium">Privacy Mode</p>
              <p className="text-xs text-[#A7B0C8]">On-device</p>
            </div>
          </div>
        </div>
        <div className="mt-6 pt-4 border-t border-white/10">
          <div className="flex items-center justify-between text-xs">
            <span className="text-[#A7B0C8]">System Load</span>
            <span className="text-[#00F0FF]">12%</span>
          </div>
          <div className="mt-2 h-1 bg-white/10 rounded-full overflow-hidden">
            <div className="h-full w-[12%] bg-gradient-to-r from-[#00F0FF] to-[#7B61FF] rounded-full" />
          </div>
        </div>
      </div>

      <div className="absolute bottom-8 left-1/2 -translate-x-1/2 flex flex-col items-center gap-2 text-[#A7B0C8]">
        <span className="text-xs font-mono uppercase tracking-wider">Scroll</span>
        <ChevronDown className="w-5 h-5 animate-bounce" />
      </div>
    </section>
  );
};

const Section2Scanning = () => {
  const sectionRef = useRef<HTMLDivElement>(null);
  const [isVisible, setIsVisible] = useState(false);
  const [progress] = useState(78);

  useEffect(() => {
    const observer = new IntersectionObserver(
      ([entry]) => {
        if (entry.isIntersecting) {
          setIsVisible(true);
        }
      },
      { threshold: 0.3 }
    );
    if (sectionRef.current) observer.observe(sectionRef.current);
    return () => observer.disconnect();
  }, []);

  return (
    <section ref={sectionRef} className="section-pinned flex items-center justify-center">
      <div className={`transition-all duration-1000 ${isVisible ? 'opacity-100 scale-100' : 'opacity-0 scale-95'}`}>
        <VideoCard>
          <div className="absolute inset-0 bg-black/40 backdrop-blur-sm flex flex-col items-center justify-center">
            <Badge className="mb-6 bg-[#00F0FF]/20 text-[#00F0FF] border-[#00F0FF]/30 px-4 py-2">
              <span className="w-2 h-2 bg-[#00F0FF] rounded-full mr-2 pulse-dot" />
              ANALYZING
            </Badge>
            
            <div className="relative w-40 h-40 mb-6">
              <svg className="w-full h-full transform -rotate-90">
                <circle
                  cx="80"
                  cy="80"
                  r="70"
                  stroke="rgba(255,255,255,0.1)"
                  strokeWidth="8"
                  fill="none"
                />
                <circle
                  cx="80"
                  cy="80"
                  r="70"
                  stroke="#00F0FF"
                  strokeWidth="8"
                  fill="none"
                  strokeLinecap="round"
                  strokeDasharray={`${2 * Math.PI * 70}`}
                  strokeDashoffset={`${2 * Math.PI * 70 * (1 - progress / 100)}`}
                  className="transition-all duration-1000"
                />
              </svg>
              <div className="absolute inset-0 flex items-center justify-center">
                <span className="text-4xl font-bold">{progress}%</span>
              </div>
            </div>
            
            <p className="text-[#A7B0C8] text-sm">Checking policy database...</p>
            <div className="flex gap-4 mt-4 text-xs text-[#A7B0C8]">
              <span className="flex items-center gap-1"><CheckCircle className="w-3 h-3 text-[#00E5A0]" /> Audio</span>
              <span className="flex items-center gap-1"><CheckCircle className="w-3 h-3 text-[#00E5A0]" /> Visuals</span>
              <span className="flex items-center gap-1"><Scan className="w-3 h-3 text-[#00F0FF] animate-pulse" /> Text</span>
            </div>
          </div>
        </VideoCard>
      </div>

      <div className={`analytics-panel transition-all duration-1000 delay-200 ${isVisible ? 'opacity-100 translate-x-0' : 'opacity-0 translate-x-20'}`}>
        <h3 className="font-mono text-xs uppercase tracking-wider text-[#A7B0C8] mb-4">Scan Progress</h3>
        <div className="space-y-4">
          <div>
            <div className="flex justify-between text-sm mb-2">
              <span>Progress</span>
              <span className="text-[#00F0FF]">{progress}%</span>
            </div>
            <Progress value={progress} className="h-2 bg-white/10" />
          </div>
          <div className="flex justify-between text-sm">
            <span className="text-[#A7B0C8]">Segments</span>
            <span>12/48</span>
          </div>
          <div className="flex justify-between text-sm">
            <span className="text-[#A7B0C8]">ETA</span>
            <span>42s</span>
          </div>
        </div>
        <div className="mt-6 pt-4 border-t border-white/10">
          <p className="text-xs text-[#A7B0C8] mb-2">Current Operation</p>
          <p className="text-sm">Audio fingerprint matching...</p>
        </div>
      </div>
    </section>
  );
};

const Section3Dashboard = () => {
  const sectionRef = useRef<HTMLDivElement>(null);
  const [isVisible, setIsVisible] = useState(false);

  useEffect(() => {
    const observer = new IntersectionObserver(
      ([entry]) => {
        if (entry.isIntersecting) {
          setIsVisible(true);
        }
      },
      { threshold: 0.3 }
    );
    if (sectionRef.current) observer.observe(sectionRef.current);
    return () => observer.disconnect();
  }, []);

  const totalRisk = 73;

  return (
    <section ref={sectionRef} className="section-pinned flex items-center justify-center">
      <div className={`transition-all duration-1000 ${isVisible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-10'}`}>
        <VideoCard>
          <div className={`absolute top-6 left-6 transition-all duration-1000 delay-300 ${isVisible ? 'opacity-100 translate-x-0' : 'opacity-0 -translate-x-10'}`}>
            <Badge className="bg-[#FF4ECD]/20 text-[#FF4ECD] border-[#FF4ECD]/30 px-4 py-2 text-base">
              <AlertTriangle className="w-4 h-4 mr-2" />
              Strike Probability: {totalRisk}%
            </Badge>
          </div>
        </VideoCard>
      </div>

      <div className={`analytics-panel transition-all duration-1000 delay-200 ${isVisible ? 'opacity-100 translate-x-0' : 'opacity-0 translate-x-20'}`}>
        <h3 className="font-mono text-xs uppercase tracking-wider text-[#A7B0C8] mb-4">Risk Breakdown</h3>
        
        <div className="relative w-32 h-32 mx-auto mb-6">
          <svg className="w-full h-full transform -rotate-90">
            {riskCategories.reduce((acc, cat, i) => {
              const prevOffset = riskCategories.slice(0, i).reduce((sum, c) => sum + c.percentage, 0);
              const circumference = 2 * Math.PI * 50;
              return [
                ...acc,
                <circle
                  key={cat.name}
                  cx="64"
                  cy="64"
                  r="50"
                  stroke={cat.color}
                  strokeWidth="12"
                  fill="none"
                  strokeLinecap="round"
                  strokeDasharray={`${circumference}`}
                  strokeDashoffset={`${circumference * (1 - cat.percentage / 100)}`}
                  style={{
                    transformOrigin: '64px 64px',
                    transform: `rotate(${prevOffset * 3.6}deg)`,
                    opacity: isVisible ? 1 : 0,
                    transition: `opacity 0.5s ease ${i * 0.1 + 0.5}s`
                  }}
                />
              ];
            }, [] as React.ReactNode[])}
          </svg>
          <div className="absolute inset-0 flex flex-col items-center justify-center">
            <span className="text-2xl font-bold">{totalRisk}%</span>
            <span className="text-xs text-[#A7B0C8]">Risk</span>
          </div>
        </div>

        <div className="space-y-3">
          {riskCategories.map((cat, i) => (
            <div 
              key={cat.name} 
              className={`flex items-center justify-between transition-all duration-500 ${isVisible ? 'opacity-100 translate-x-0' : 'opacity-0 translate-x-4'}`}
              style={{ transitionDelay: `${i * 0.1 + 0.6}s` }}
            >
              <div className="flex items-center gap-2">
                <div className="w-3 h-3 rounded-full" style={{ background: cat.color }} />
                <span className="text-sm">{cat.name}</span>
              </div>
              <span className="text-sm font-medium" style={{ color: cat.color }}>{cat.percentage}%</span>
            </div>
          ))}
        </div>

        <div className="mt-6 pt-4 border-t border-white/10">
          <p className="text-xs text-[#A7B0C8] mb-1">Highest Risk</p>
          <p className="text-sm text-[#FF4ECD]">Audio copyright violation detected</p>
        </div>
      </div>
    </section>
  );
};

const Section4Timeline = () => {
  const sectionRef = useRef<HTMLDivElement>(null);
  const [isVisible, setIsVisible] = useState(false);
  const [selectedSegment, setSelectedSegment] = useState<string>('1');

  useEffect(() => {
    const observer = new IntersectionObserver(
      ([entry]) => {
        if (entry.isIntersecting) {
          setIsVisible(true);
        }
      },
      { threshold: 0.3 }
    );
    if (sectionRef.current) observer.observe(sectionRef.current);
    return () => observer.disconnect();
  }, []);

  return (
    <section ref={sectionRef} className="section-pinned flex items-center justify-center">
      <div className={`transition-all duration-1000 ${isVisible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-10'}`}>
        <VideoCard>
          <div className={`absolute bottom-0 left-0 right-0 h-[15%] bg-gradient-to-t from-black/90 to-transparent transition-all duration-1000 ${isVisible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-10'}`}>
            <div className="absolute bottom-4 left-4 right-4">
              <div className="flex items-center gap-2 mb-2">
                <span className="text-xs font-mono text-[#A7B0C8]">00:00</span>
                <div className="flex-1 h-8 bg-white/5 rounded-lg overflow-hidden flex relative">
                  <div className="w-[28%] bg-[#00F0FF]/30" />
                  <div className="w-[12%] bg-[#FF4ECD]/60" />
                  <div className="w-[15%] bg-[#00F0FF]/30" />
                  <div className="w-[8%] bg-[#FFD166]/60" />
                  <div className="flex-1 bg-[#00F0FF]/30" />
                  <div className="w-[6%] bg-[#00E5A0]/40" />
                  <div className="absolute left-[34%] top-0 bottom-0 w-0.5 bg-white shadow-[0_0_10px_rgba(255,255,255,0.8)]" />
                </div>
                <span className="text-xs font-mono text-[#A7B0C8]">03:24</span>
              </div>
              
              {selectedSegment === '1' && (
                <div className="absolute -top-12 left-[30%] bg-[#FF4ECD] text-black text-xs font-medium px-3 py-1.5 rounded-lg">
                  00:34–00:48
                  <div className="absolute -bottom-1 left-1/2 -translate-x-1/2 w-2 h-2 bg-[#FF4ECD] rotate-45" />
                </div>
              )}
            </div>
          </div>
        </VideoCard>
      </div>

      <div className={`analytics-panel transition-all duration-1000 delay-200 ${isVisible ? 'opacity-100 translate-x-0' : 'opacity-0 translate-x-20'}`}>
        <h3 className="font-mono text-xs uppercase tracking-wider text-[#A7B0C8] mb-4">Flagged Segments</h3>
        <div className="space-y-3">
          {flaggedSegments.map((segment, i) => (
            <div
              key={segment.id}
              onClick={() => setSelectedSegment(segment.id)}
              className={`p-3 rounded-xl cursor-pointer transition-all duration-300 ${
                selectedSegment === segment.id 
                  ? 'bg-white/10 border border-[#00F0FF]/30' 
                  : 'bg-white/5 hover:bg-white/8 border border-transparent'
              } ${isVisible ? 'opacity-100 translate-x-0' : 'opacity-0 translate-x-4'}`}
              style={{ transitionDelay: `${i * 0.1 + 0.4}s` }}
            >
              <div className="flex items-center justify-between mb-1">
                <span className="text-sm font-medium">Segment {String.fromCharCode(65 + i)}</span>
                <Badge 
                  className={`text-xs ${
                    segment.severity === 'High' ? 'bg-[#FF4ECD]/20 text-[#FF4ECD]' :
                    segment.severity === 'Medium' ? 'bg-[#FFD166]/20 text-[#FFD166]' :
                    'bg-[#00E5A0]/20 text-[#00E5A0]'
                  }`}
                >
                  {segment.severity}
                </Badge>
              </div>
              <p className="text-xs text-[#A7B0C8] font-mono">{segment.start}–{segment.end}</p>
              <p className="text-xs text-[#A7B0C8] mt-1">{segment.issue}</p>
            </div>
          ))}
        </div>
        <p className="mt-4 text-xs text-[#A7B0C8]">Click any segment to preview the fix</p>
      </div>
    </section>
  );
};

const Section5Policy = () => {
  const sectionRef = useRef<HTMLDivElement>(null);
  const [isVisible, setIsVisible] = useState(false);
  const [selectedOption, setSelectedOption] = useState<string>('1');

  useEffect(() => {
    const observer = new IntersectionObserver(
      ([entry]) => {
        if (entry.isIntersecting) {
          setIsVisible(true);
        }
      },
      { threshold: 0.3 }
    );
    if (sectionRef.current) observer.observe(sectionRef.current);
    return () => observer.disconnect();
  }, []);

  return (
    <section ref={sectionRef} className="section-pinned flex items-center justify-center">
      <div className={`transition-all duration-1000 ${isVisible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-10'}`}>
        <VideoCard>
          <div className={`absolute bottom-6 left-6 max-w-xs glass-card p-4 transition-all duration-1000 delay-300 ${isVisible ? 'opacity-100 translate-x-0' : 'opacity-0 -translate-x-10'}`}>
            <div className="flex items-center gap-2 mb-3">
              <FileText className="w-4 h-4 text-[#00F0FF]" />
              <h4 className="text-sm font-medium">YouTube Community Guidelines</h4>
            </div>
            <ul className="space-y-2 text-xs text-[#A7B0C8]">
              <li className="flex items-start gap-2">
                <AlertTriangle className="w-3 h-3 text-[#FFD166] mt-0.5 flex-shrink-0" />
                <span>Copyright: Using rights-managed audio without a license</span>
              </li>
              <li className="flex items-start gap-2">
                <AlertTriangle className="w-3 h-3 text-[#FF4ECD] mt-0.5 flex-shrink-0" />
                <span>Harmful content: Graphic imagery without context</span>
              </li>
            </ul>
            <a href="#" className="text-xs text-[#00F0FF] hover:underline mt-3 inline-block">Read full policy →</a>
          </div>
        </VideoCard>
      </div>

      <div className={`analytics-panel transition-all duration-1000 delay-200 ${isVisible ? 'opacity-100 translate-x-0' : 'opacity-0 translate-x-20'}`}>
        <h3 className="font-mono text-xs uppercase tracking-wider text-[#A7B0C8] mb-4">Remediation Options</h3>
        <div className="space-y-3">
          {remediationOptions.map((option, i) => (
            <div
              key={option.id}
              onClick={() => setSelectedOption(option.id)}
              className={`p-3 rounded-xl cursor-pointer transition-all duration-300 ${
                selectedOption === option.id 
                  ? 'bg-[#00F0FF]/10 border border-[#00F0FF]/30' 
                  : 'bg-white/5 hover:bg-white/8 border border-transparent'
              } ${isVisible ? 'opacity-100 translate-x-0' : 'opacity-0 translate-x-4'}`}
              style={{ transitionDelay: `${i * 0.1 + 0.4}s` }}
            >
              <div className="flex items-center gap-3">
                <div className={`w-4 h-4 rounded-full border-2 flex items-center justify-center ${
                  selectedOption === option.id ? 'border-[#00F0FF]' : 'border-white/30'
                }`}>
                  {selectedOption === option.id && <div className="w-2 h-2 bg-[#00F0FF] rounded-full" />}
                </div>
                <div className="flex-1">
                  <div className="flex items-center justify-between">
                    <span className="text-sm font-medium">{option.title}</span>
                    <Badge 
                      className={`text-xs ${
                        option.impact === 'Low' ? 'bg-[#00E5A0]/20 text-[#00E5A0]' :
                        option.impact === 'Medium' ? 'bg-[#FFD166]/20 text-[#FFD166]' :
                        'bg-[#FF4ECD]/20 text-[#FF4ECD]'
                      }`}
                    >
                      Impact: {option.impact}
                    </Badge>
                  </div>
                  <p className="text-xs text-[#A7B0C8] mt-0.5">{option.description}</p>
                </div>
              </div>
            </div>
          ))}
        </div>
      </div>
    </section>
  );
};

const Section6AutoFix = () => {
  const sectionRef = useRef<HTMLDivElement>(null);
  const [isVisible, setIsVisible] = useState(false);
  const [intensity, setIntensity] = useState(72);
  const [quality, setQuality] = useState(85);
  const [preserveAudio, setPreserveAudio] = useState(true);

  useEffect(() => {
    const observer = new IntersectionObserver(
      ([entry]) => {
        if (entry.isIntersecting) {
          setIsVisible(true);
        }
      },
      { threshold: 0.3 }
    );
    if (sectionRef.current) observer.observe(sectionRef.current);
    return () => observer.disconnect();
  }, []);

  return (
    <section ref={sectionRef} className="section-pinned flex items-center justify-center">
      <div className={`transition-all duration-1000 ${isVisible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-10'}`}>
        <VideoCard showDivider image="/video_thumb_cyberpunk.jpg">
          <div className="absolute top-6 left-6">
            <Badge className="bg-white/10 text-white border-white/20 px-3 py-1">
              Original
            </Badge>
          </div>
          <div className="absolute top-6 right-6">
            <Badge className="bg-[#00F0FF]/20 text-[#00F0FF] border-[#00F0FF]/30 px-3 py-1">
              Proposed Fix
            </Badge>
          </div>
          <div className={`absolute top-6 left-1/2 -translate-x-1/2 transition-all duration-1000 delay-500 ${isVisible ? 'opacity-100 translate-y-0' : 'opacity-0 -translate-y-4'}`}>
            <Badge className="bg-[#00E5A0]/20 text-[#00E5A0] border-[#00E5A0]/30 px-4 py-2">
              <CheckCircle className="w-4 h-4 mr-2" />
              Confidence: 96%
            </Badge>
          </div>
          <div className="absolute inset-0 pointer-events-none">
            <div className="absolute left-1/2 top-0 bottom-0 w-px bg-[#00F0FF] shadow-[0_0_20px_rgba(0,240,255,0.5)]" />
          </div>
          <div className="absolute right-0 top-0 bottom-0 w-1/2 overflow-hidden">
            <img 
              src="/video_thumb_fixed.jpg" 
              alt="Fixed preview" 
              className="w-[200%] h-full object-cover"
              style={{ marginLeft: '-100%' }}
            />
            <div className="absolute inset-0 bg-[#00F0FF]/5" />
          </div>
        </VideoCard>
      </div>

      <div className={`analytics-panel transition-all duration-1000 delay-200 ${isVisible ? 'opacity-100 translate-x-0' : 'opacity-0 translate-x-20'}`}>
        <h3 className="font-mono text-xs uppercase tracking-wider text-[#A7B0C8] mb-4">Fix Settings</h3>
        
        <div className="space-y-5">
          <div>
            <div className="flex justify-between text-sm mb-2">
              <span>Intensity</span>
              <span className="text-[#00F0FF]">{intensity}%</span>
            </div>
            <Slider 
              value={[intensity]} 
              onValueChange={([v]) => setIntensity(v)} 
              max={100} 
              step={1}
              className="cursor-pointer"
            />
          </div>
          
          <div>
            <div className="flex justify-between text-sm mb-2">
              <span>Quality</span>
              <span className="text-[#7B61FF]">{quality >= 80 ? 'High' : quality >= 50 ? 'Medium' : 'Fast'}</span>
            </div>
            <Slider 
              value={[quality]} 
              onValueChange={([v]) => setQuality(v)} 
              max={100} 
              step={1}
              className="cursor-pointer"
            />
          </div>
          
          <div 
            onClick={() => setPreserveAudio(!preserveAudio)}
            className="flex items-center justify-between p-3 bg-white/5 rounded-xl cursor-pointer hover:bg-white/8 transition-colors"
          >
            <div className="flex items-center gap-2">
              {preserveAudio ? <Volume2 className="w-4 h-4 text-[#00E5A0]" /> : <VolumeX className="w-4 h-4 text-[#A7B0C8]" />}
              <span className="text-sm">Preserve Audio</span>
            </div>
            <div className={`w-10 h-5 rounded-full transition-colors ${preserveAudio ? 'bg-[#00E5A0]' : 'bg-white/20'}`}>
              <div className={`w-4 h-4 bg-white rounded-full mt-0.5 transition-transform ${preserveAudio ? 'translate-x-5' : 'translate-x-0.5'}`} />
            </div>
          </div>
        </div>

        <div className="mt-6 space-y-2">
          <Button className="w-full bg-[#00F0FF] text-black hover:bg-[#00F0FF]/90 rounded-xl py-5 font-medium">
            <CheckCircle className="w-4 h-4 mr-2" />
            Apply Fix
          </Button>
          <Button variant="outline" className="w-full border-white/20 hover:bg-white/5 rounded-xl py-5">
            <Download className="w-4 h-4 mr-2" />
            Download Report
          </Button>
        </div>
      </div>
    </section>
  );
};

const Section7Features = () => {
  const sectionRef = useRef<HTMLDivElement>(null);
  const [isVisible, setIsVisible] = useState(false);

  useEffect(() => {
    const observer = new IntersectionObserver(
      ([entry]) => {
        if (entry.isIntersecting) {
          setIsVisible(true);
        }
      },
      { threshold: 0.2 }
    );
    if (sectionRef.current) observer.observe(sectionRef.current);
    return () => observer.disconnect();
  }, []);

  const features = [
    { icon: Lock, title: 'On-device analysis', description: 'Your content never leaves your machine. Full privacy with local AI processing.' },
    { icon: Zap, title: 'Real-time policy updates', description: 'Automatic adjustment when YouTube changes guidelines. Always stay compliant.' },
    { icon: Eye, title: 'Professional-grade fixes', description: 'AI-powered edits indistinguishable from manual professional editing.' },
  ];

  return (
    <section ref={sectionRef} className="min-h-screen py-24 px-8 bg-[#0B0E17]">
      <div className="max-w-4xl mx-auto">
        <div className={`text-center mb-16 transition-all duration-1000 ${isVisible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-10'}`}>
          <h2 className="text-4xl md:text-5xl font-bold mb-4">
            Built for creators.<br />
            <span className="text-gradient">Engineered for trust.</span>
          </h2>
        </div>

        <div className="grid md:grid-cols-3 gap-6 mb-16">
          {features.map((feature, i) => (
            <div 
              key={feature.title}
              className={`glass-card p-6 transition-all duration-700 ${isVisible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-10'}`}
              style={{ transitionDelay: `${i * 0.15 + 0.3}s` }}
            >
              <div className="w-12 h-12 rounded-xl bg-[#00F0FF]/10 flex items-center justify-center mb-4">
                <feature.icon className="w-6 h-6 text-[#00F0FF]" />
              </div>
              <h3 className="text-lg font-semibold mb-2">{feature.title}</h3>
              <p className="text-sm text-[#A7B0C8]">{feature.description}</p>
            </div>
          ))}
        </div>

        <div className={`glass-card p-8 transition-all duration-1000 delay-700 ${isVisible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-10'}`}>
          <h3 className="font-mono text-xs uppercase tracking-wider text-[#A7B0C8] mb-6">Technical Specifications</h3>
          <div className="grid grid-cols-2 md:grid-cols-4 gap-6">
            <div className="text-center">
              <Cpu className="w-6 h-6 text-[#00F0FF] mx-auto mb-2" />
              <p className="text-sm font-medium">ARM / x86_64</p>
              <p className="text-xs text-[#A7B0C8]">Dual architecture</p>
            </div>
            <div className="text-center">
              <Zap className="w-6 h-6 text-[#7B61FF] mx-auto mb-2" />
              <p className="text-sm font-medium">Vulkan GPU</p>
              <p className="text-xs text-[#A7B0C8]">Cross-platform</p>
            </div>
            <div className="text-center">
              <BarChart3 className="w-6 h-6 text-[#FF4ECD] mx-auto mb-2" />
              <p className="text-sm font-medium">≤4GB RAM</p>
              <p className="text-xs text-[#A7B0C8]">1080p analysis</p>
            </div>
            <div className="text-center">
              <Shield className="w-6 h-6 text-[#00E5A0] mx-auto mb-2" />
              <p className="text-sm font-medium">Zero Trust</p>
              <p className="text-xs text-[#A7B0C8]">Local-only</p>
            </div>
          </div>
        </div>
      </div>
    </section>
  );
};

const Section8CTA = () => {
  const sectionRef = useRef<HTMLDivElement>(null);
  const [isVisible, setIsVisible] = useState(false);

  useEffect(() => {
    const observer = new IntersectionObserver(
      ([entry]) => {
        if (entry.isIntersecting) {
          setIsVisible(true);
        }
      },
      { threshold: 0.2 }
    );
    if (sectionRef.current) observer.observe(sectionRef.current);
    return () => observer.disconnect();
  }, []);

  return (
    <section ref={sectionRef} className="min-h-screen flex flex-col items-center justify-center py-24 px-8">
      <div className={`text-center max-w-2xl mx-auto mb-12 transition-all duration-1000 ${isVisible ? 'opacity-100 scale-100' : 'opacity-0 scale-95'}`}>
        <h2 className="text-4xl md:text-5xl font-bold mb-4">
          Ready to protect<br />
          <span className="text-gradient">your channel?</span>
        </h2>
        <p className="text-lg text-[#A7B0C8] mb-8">
          Start free. Upgrade when you need more power.
        </p>
        <div className="flex flex-col sm:flex-row gap-4 justify-center">
          <Button className="bg-[#00F0FF] text-black hover:bg-[#00F0FF]/90 rounded-full px-8 py-6 text-base font-medium">
            Start Free Scan
            <ArrowRight className="w-5 h-5 ml-2" />
          </Button>
          <Button variant="outline" className="border-white/20 hover:bg-white/5 rounded-full px-8 py-6 text-base">
            View Pricing
          </Button>
        </div>
      </div>

      <footer className={`w-full max-w-4xl mx-auto pt-16 border-t border-white/10 transition-all duration-1000 delay-300 ${isVisible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-10'}`}>
        <div className="flex flex-wrap justify-center gap-8 mb-8">
          <a href="#" className="text-sm text-[#A7B0C8] hover:text-white transition-colors">Product</a>
          <a href="#" className="text-sm text-[#A7B0C8] hover:text-white transition-colors">Demo</a>
          <a href="#" className="text-sm text-[#A7B0C8] hover:text-white transition-colors">Pricing</a>
          <a href="#" className="text-sm text-[#A7B0C8] hover:text-white transition-colors">Docs</a>
          <a href="#" className="text-sm text-[#A7B0C8] hover:text-white transition-colors">Privacy</a>
          <a href="#" className="text-sm text-[#A7B0C8] hover:text-white transition-colors">Terms</a>
        </div>
        <p className="text-center text-xs text-[#A7B0C8]">© 2026 ContentGuard AI. All rights reserved.</p>
      </footer>
    </section>
  );
};

function App() {
  return (
    <div className="relative">
      <div className="grain-overlay" />
      <div className="vignette" />
      <Navigation />
      
      <main className="relative">
        <Section1Hero />
        <Section2Scanning />
        <Section3Dashboard />
        <Section4Timeline />
        <Section5Policy />
        <Section6AutoFix />
        <Section7Features />
        <Section8CTA />
      </main>
    </div>
  );
}

export default App;
