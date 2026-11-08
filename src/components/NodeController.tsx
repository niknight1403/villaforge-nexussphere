import React, { useState } from 'react';

export const NodeController: React.FC = () => {
  const [status, setStatus] = useState<'idle' | 'active' | 'error'>('idle');

  return (
    <div className="p-6 max-w-md mx-auto bg-white rounded-xl shadow-md space-y-4">
      <h1 className="text-xl font-bold text-gray-900">NexusSphere Node Controller</h1>
      <div className="flex items-center space-x-4">
        <span className={`px-3 py-1 rounded-full text-sm ${status === 'active' ? 'bg-green-100 text-green-800' : 'bg-gray-100 text-gray-800'}`}>
          Status: {status.toUpperCase()}
        </span>
      </div>
      <button 
        onClick={() => setStatus('active')}
        className="w-full bg-blue-600 text-white py-2 px-4 rounded hover:bg-blue-700 transition"
      >
        Initialize Node
      </button>
    </div>
  );
};