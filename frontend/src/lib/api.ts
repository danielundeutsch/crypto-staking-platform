import axios from 'axios';

const API_BASE_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8080/api/v1';

export const api = axios.create({
  baseURL: API_BASE_URL,
  headers: {
    'Content-Type': 'application/json',
  },
});

export interface StakeRequest {
  amount: string;
  address: string;
  duration_days?: number;
}

export interface StakeResponse {
  transaction_id: string;
  status: string;
  estimated_rewards: string;
}

export interface BalanceResponse {
  address: string;
  balance: string;
  staked_amount: string;
  pending_rewards: string;
}

export interface Service {
  name: string;
  status: 'healthy' | 'unhealthy';
  chain: string;
}

export const stakingAPI = {
  stake: async (chain: string, data: StakeRequest): Promise<StakeResponse> => {
    const response = await api.post(`/stake/${chain}`, data);
    return response.data;
  },

  getBalance: async (chain: string, address: string): Promise<BalanceResponse> => {
    const response = await api.get(`/balance/${chain}/${address}`);
    return response.data;
  },

  getServices: async (): Promise<Service[]> => {
    const response = await api.get('/services');
    return response.data;
  },
};