class Twitter:

    def __init__(self):
        self.userFollowing = {}
        self.userTweet = {}
        self.time = 1

    def postTweet(self, userId: int, tweetId: int) -> None:
        if userId in self.userTweet:
            heap = self.userTweet[userId]
        else:
            heap = []
            self.userTweet[userId] = heap
        heap.append((-self.time, tweetId))
        self.time += 1
        if userId in self.userFollowing:
            heap = self.userFollowing[userId]
        else:
            heap = {}
            self.userFollowing[userId] = heap
        heap[userId] = 0

    def getNewsFeed(self, userId: int) -> List[int]:
        keys = self.userFollowing[userId].keys() if self.userFollowing else []
        arr = []
        res = []
        for k in keys:
            arr = [*arr, *self.userTweet[k]]
        heapq.heapify(arr)
        while arr and len(res) < 10:
            res.append(heapq.heappop(arr)[1])
        return res
        

    def follow(self, followerId: int, followeeId: int) -> None:
        if followerId in self.userFollowing:
            heap = self.userFollowing[followerId]
        else:
            heap = {}
            self.userFollowing[followerId] = heap
        heap[followeeId] = 0
        

    def unfollow(self, followerId: int, followeeId: int) -> None:
        if followerId == followeeId:
            return
        if followerId in self.userFollowing:
            heap = self.userFollowing[followerId]
        else:
            heap = {}
            self.userFollowing[followerId] = heap
        if followeeId in heap:
            heap.pop(followeeId)
