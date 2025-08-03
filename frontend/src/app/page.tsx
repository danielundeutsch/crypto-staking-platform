import { Header } from "@/components/Header";
import { StakingDashboard } from "@/components/StakingDashboard";

export default function Home() {
  return (
    <div className="min-h-screen bg-gray-50">
      <Header />
      <main>
        <StakingDashboard />
      </main>
    </div>
  );
}
