class Twitter:

    def __init__(self):
        self.userFollowing = defaultdict(set)
        self.userTweet = defaultdict(list)
        self.time = 1


    def postTweet(self, userId: int, tweetId: int) -> None:
        self.userTweet[userId].append((-self.time, tweetId))
        self.time += 1


    def getNewsFeed(self, userId: int) -> List[int]:
        keys = self.userFollowing[userId]
        keys.add(userId)
        arr, res = [], []
        for k in keys:
            arr = [*arr, *self.userTweet[k]]
        heapq.heapify(arr)
        while arr and len(res) < 10:
            res.append(heapq.heappop(arr)[1])
        return res
        

    def follow(self, followerId: int, followeeId: int) -> None:
        self.userFollowing[followerId].add(followeeId)
        

    def unfollow(self, followerId: int, followeeId: int) -> None:
        if followeeId in self.userFollowing[followerId]:
            self.userFollowing[followerId].remove(followeeId)
