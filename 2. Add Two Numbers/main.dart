class Solution {
  List<int> twoSum(List<int> nums, int target) {
    Map<int, int> complementMap = {};

    for (var i = 0; i < nums.length; i++) {
      final complement = target - nums[i];
      final int? index = complementMap[complement];
      if (index != null) {
        return [index, i];
      }
      complementMap[nums[i]] = i;
    }

    return new List.empty();
  }
}
