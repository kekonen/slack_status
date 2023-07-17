import subprocess, time

text_lines = [
    "There's a party goin' on right here",
    "A celebration to last throughout the years",
    "So bring your good times and your laughter too",
    "We gonna celebrate your party with you",
    "Come on now, celebration",
    "Let's all celebrate and have a good time",
    "Celebration",
    "We gonna celebrate and have a good time",

    "Yahoo!",
    "Celebration",
    "Yahoo!",
    "This is your celebration",
    "Celebrate good times, come on!",
    "(Let's celebrate)",
    "Celebrate good times, come on!",
    "(Let's celebrate)",
    "There's a party goin' on right here",
    "A celebration to last throughout the years",
    "So bring your good times and your laughter too",
    "We gonna celebrate your party with you",
    "Come on now, celebration",
    "Let's all celebrate and have a good time",
    "Celebration",
    "We gonna celebrate and have a good time",
    "It's time to come together",
    "It's up to you, what's your pleasure?",
    "Everyone around the world come on!",
    "Yahoo!",
    "It's a celebration",
    "Yahoo!",
    "Celebrate good times, come on!",
    "(It's a celebration)",
    "Celebrate good times, come on!",
    "(Let's celebrate)",
    "There's a party goin' on right here",
    "A dedication to last throughout the years",
    "So bring your good times and your laughter too",
    "We gonna celebrate and party with you",
    "Come on now, celebration",
    "Let's all celebrate and have a good time, yeah yeah",
    "Celebration",
    "We gonna celebrate and have a good time",
    "It's time to come together",
    "It's up to you, what's your pleasure?",
    "Everyone around the world come on!",
    "Yahoo!",
    "It's a celebration",
    "Yahoo!",
    "It's a celebration",
    "Celebrate good times, come on!",
    "(Let's celebrate come on now)",
    "Celebrate good times, come on!",
    "(Let's celebrate)",
    "We're gonna have a good time tonight",
    "Let's celebrate, it's all right",
    "We're gonna have a good time tonight",
    "Let's celebrate, it's all right, baby",
    "We're gonna have a good time tonight",
    "(Celebration)",
    "Let's celebrate, it's all right",
    "We're gonna have a good time tonight",
    "(Celebration)",
    "Let's celebrate, it's all right",
    "Yahoo!",
    "Yahoo!",
    "Celebrate good times, come on!",
    "(Let's celebrate)",
    "Celebrate good times, come on!",
    "(It's a celebration)",
    "Celebrate good times, come on!",
    "(Let's celebrate)",
    "(Come on and celebrate tonight)",
    "Celebrate good times, come on!",
    "('Cause everything's gonna be alright, let's celebrate)",
    "Celebrate good times, come on!",
    "(Let's celebrate)",
    "Celebrate good times, come on!",
]

emojis = [
    "tada",
    "partyparrot",
    "man_dancing",
    "dancer",
    "mirror_ball",
    "partying_face",
    "clinking_glasses",
    "champagne",
]

class Roller:
    inner = None
    state = 0
    def __init__(self, inner):
        self.inner = inner

    def next(self):
        self.state += 1
        if self.state >= len(self.inner):
            self.state = 0

        return self.inner[self.state]


song_roller = Roller(text_lines)
emoji_roller = Roller(emojis)

seconds = 10

for x in range(0, 10000):
    # wait some time
    time.sleep(seconds)
    try:
        subprocess.call(["slack_update", "status", "-e", ':{}:'.format(emoji_roller.next()), "-t", '"{}"'.format(song_roller.next())])
    except e:
        print("Error", e)
        if e == "ratelimited":
            time.sleep(seconds * 2)
        pass

