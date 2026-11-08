import React from 'react';
import { View, Text } from 'react-native';
import { TailwindProvider } from 'tailwind-rn';

const NodeController = () => {
  return (
    <TailwindProvider>
      <View className='flex-1 items-center justify-center'>
        <Text className='text-2xl font-bold'>Node Controller</Text>
      </View>
    </TailwindProvider>
  );
};

export default NodeController;