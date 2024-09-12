def merge_alternatively(word1: String, word2: String):
    # we only need to look at both words for as long as the smallest word
    l = min(len(word1), len(word2))

    ret = ""

    for i in range(l):
        # add alternating
        ret += word1[i] + word2[i]

    # if word1 was smaller then word1[l:] = word1[len(word1):] = ""
    # and so adding it does nothing
    # so word1[l:] + word2[l:] would just be the parts of word2 that we havent seen yet

    # similar if word2 smaller
    return ret + word1[l:] + word2[l:]
