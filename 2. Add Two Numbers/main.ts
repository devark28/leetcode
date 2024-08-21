function twoSum(nums: number[], target: number): number[] {
    const complementMap = new Map<number, number>();

    for (let i = 0; i < nums.length; i++) {
        const complement = target - nums[i];
        if (complementMap.has(complement)) {
            return [complementMap.get(complement)!, i];
        }
        complementMap.set(nums[i], i);
    }

    return [];
}