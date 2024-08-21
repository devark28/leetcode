class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        complementDict = {}
        for index, num in enumerate(nums):
            compliment = target - num
            if compliment in complementDict:
                return [complementDict[compliment], index]
            else:
                complementDict[num] = index
        return []
        
