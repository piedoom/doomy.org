+++
title = "Publishing asks with your Tumblr bot"
date = 2016-08-02
author = "doomy"

[taxonomies]
categories = ["tutorial"]

[extra]
tags = ["programming", "ruby", "tumblr", "bot"]
+++

### Required Reading

If you haven't done so already, please read [Creating a Tumblr Bot from Scratch](https://vaporsoft.net/creating-a-tumblr-bot-from-scratch/).  This tutorial will bootstrap off of what we did last time.  If you're super positive you don't need to do that tutorial, or lost your files somewhere in the depths of your harddrive, you can download a copy here.

<a href="/files/creating-a-tumblr-bot-from-scratch/creating-a-tumblr-bot-from-scratch.zip" class="button">Download Starter Files</a>

### End Goal

We're going to create a Tumblr bot that answers people's asks, similar to the infamous [doomybot](http://bot.doomy.me/).  We're also going to respond contextually depending on the user's ask (although it will be done crudely).

### Note to Windows Users

Since August 2nd, 2016, Microsoft has released the Windows 10 anniversary edition.  As you might know, I don't provide per-tutorial support to Windows users.  However, in this new update, Microsoft released the Linux subsystem for Windows, which is a fancy way of saying you can use all of the commands I use in Linux on your Windows PC with no extra software.  At this point, I'm still fond of [Cygwin](https://cygwin.com/) or running *nix on a VM, but you might like Microsoft's solution better.  To enable this feature, search "Turn Windows Features on and off". Scroll down and check "Windows Subsystem for Linux" (If you can't find this option, you're not on the most recent version of Windows 10).  You'll need to restart your computer after this for the changes to take effect.  Once restarted, if you search "bash" from the start menu, you should get a bash terminal (basically the same terminal I use for these tutorials).  

### Getting Started

Let's open our `tumblr_bot.rb` file.  It should look something like this:

```ruby
require 'tumblr_client'
def post_a_thing

    # Authenticate via OAuth
    client = Tumblr::Client.new({
    :consumer_key => 'xxxxxxxxxxxxxxxxxxxx',
    :consumer_secret => 'xxxxxxxxxxxxxxxxxxxx',
    :oauth_token => 'xxxxxxxxxxxxxxxxxxxx',
    :oauth_token_secret => 'xxxxxxxxxxxxxxxxxxxx'
    })

    alignments = ["Lawful Good", 
                    "Neutral Good", 
                    "Chaotic Good", 
                    "Lawful Neutral",
                    "True Neutral",
                    "Chaotic Neutral",
                    "Lawful Evil",
                    "Neutral Evil",
                    "Chaotic Evil" ]

    adjectives = ["Cool", "Gamer", "Doctor", "Pathetic"]
    nouns = ["Boy", "Pineapple Ham", "Child", "Gluestick", "Muffin"]

    sentence = "#{alignments.sample}: #{adjectives.sample} #{nouns.sample}."

    # Make the request
    client.text("your_blog", title: sentence)
end
```

As a refresher, our interaction with Tumblr happens at the penultimate line: `client.text("your_blog", title: sentence)`.  We call the `text` function on our Tumblr `client` object, and create a text post.

Let's go ahead and delete some lines that we don't need right now.  (You can make a copy of the project if you want to save your old bot).  All we want to keep for now is the `client` creation.

```ruby
require 'tumblr_client'
def post_a_thing

    # Authenticate via OAuth
    client = Tumblr::Client.new({
    :consumer_key => 'xxxxxxxxxxxxxxxxxxxx',
    :consumer_secret => 'xxxxxxxxxxxxxxxxxxxx',
    :oauth_token => 'xxxxxxxxxxxxxxxxxxxx',
    :oauth_token_secret => 'xxxxxxxxxxxxxxxxxxxx'
    })
    
end
```


Great job, you just deleted *hours* of work.

### Secure your Stuff

If you're using Windows, don't worry about this part - but still read through it as code snippets in the future will reference this.

Anyways, right now we have this code:

```ruby
client = Tumblr::Client.new({
    :consumer_key => 'xxxxxxxxxxxxxxxxxxxx',
    :consumer_secret => 'xxxxxxxxxxxxxxxxxxxx',
    :oauth_token => 'xxxxxxxxxxxxxxxxxxxx',
    :oauth_token_secret => 'xxxxxxxxxxxxxxxxxxxx'
    })
```

Yours doesn't look like this, exactly.  You have a bunch of random looking characters where the X's are currently.  The thing is, those random characters contain extremely sensitive information.  If someone gained access to those strings, they could take over your Tumblr account.  You might be thinking, "Well, I just won't share this code with anyone!", and that's a valid thought.  However, it's best practice to store your super-secret password stuff in other places so you don't [accidentally happen to upload it to a very public website](https://github.com/search?q=oauth_token_secret&type=Code&utf8=%E2%9C%93).

We can do this by using environment variables.  If you're using linux, open up the file located at `~/.bashrc`.  If you're on a Mac, the process seems to be slightly different.  I don't have a Mac I can test with, but I believe the equivalent file is `~/.bash_profile`.

> If you have a different shell installed like ZSH, your file will be located at `.zshrc` - but if you went out of your way to install a different shell, you probably know how to set variables already.

Lets open the file, and scroll down to the bottom if there's already stuff written.  We're going to add the following four lines:

```ruby
export CONSUMER_KEY=""
export CONSUMER_SECRET=""
export OAUTH_TOKEN=""
export OAUTH_TOKEN_SECRET=""
```

You might notice that these four variables are the same as the ones defined within our Tumblr `client` in our `tumblr_bot.rb` file.  There is a correlation!  The variable names are the same, except they're all uppercase in our `~/.bashrc` (as is the convention).  Let's copy and paste the keys within the `'` characters from our `tumblr_bot.rb` file between the `"` characters in our `~/.bashrc` file. 

Example:

If your `tumblr_bot.rb` file looks like this...

```ruby
:consumer_key => 'aaabbbccc',
:consumer_secret => 'gghhjj',
:oauth_token => 'wweerr',
:oauth_token_secret => 'uuiioo'
```

... the end of your `~/.bashrc` file should look like this:


```bash
export CONSUMER_KEY="aaabbbccc"
export CONSUMER_SECRET="gghhjj"
export OAUTH_TOKEN="wweerr"
export OAUTH_TOKEN_SECRET="uuiioo"
```

Great!  We've stored our environment variables.  We need to reload our shell for the changes to take effect.  You can do this by shutting down your terminal and opening a new one, or by typing `source ~/.bashrc`.

We need to make one last change to our `tumblr_bot.rb` file in order for it to use the environment variables we just set.  We'll use a built-in Ruby feature that fetches environment variables with the following - 

```ruby
client = Tumblr::Client.new({
    :consumer_key => ENV['CONSUMER_KEY'],
    :consumer_secret => ENV['CONSUMER_SECRET'],
    :oauth_token => ENV['OAUTH_TOKEN'],
    :oauth_token_secret => ENV['OAUTH_TOKEN_SECRET']
})
```

That's it!  Now we can share our whole project folder with others.  Note that I'll be using the `ENV[]` syntax in examples below, so don't get confused if you didn't do this step, and my stuff looks different than yours.

### Modifying our Timing

Anyways, lets open up our `bot.rb` file in the `bin` folder.  It should look like this:

```ruby
require_relative '../lib/tumblr_bot.rb'

while (true)
    post_a_thing
    puts 'posted a thing'
    sleep 3600
end
```

Right now, we're posting a thing every `3600` seconds, which translates to 1 hour.  We're going to change this number down to about `60` seconds.

#### So, why the change?

In the previous version of our bot, it posted completely on its own, without any influence from the outside world.  However, in our new version, we're responding to asks.  Since Tumblr doesn't provide [webhooks](https://en.wikipedia.org/wiki/Webhook) when someone sends an ask, we need to constantly check our inbox to see if we got any new messages.  Think of it like checking your email every minute.

#### Okay, so why `60`?

60 seconds is admittedly an arbitrary number.  `61` will work just as much as `32`.  However, I recommend not using super-small numbers like `10` seconds.  This is because Tumblr would have to deal with your request every 10 seconds, which is *probably* against their terms of service.  Be respectful to the service you're using!

Anyways, our line should now read 
```ruby
sleep 60
```

### Checking for asks

Okay, remember when I said to be respectful to the service you're using?  Well I'm going to be hypocritcal here and say that Tumblr's system for responding to asks via the API is plain *stupid*.  The feature isn't documented anywhere, and I had to reverse-engineer the official app to figure out what [endpoints](http://stackoverflow.com/a/18768849/2584408) were being called. 

With that being said, I've made a little diagram of how our code flow is going to look.  

![](/images/publishing-asks-with-your-tumblr-bot-2/tumblr-bot-diagram-01-01-01.png)

We're getting all submissions.  This includes your entire inbox - submission posts and asks alike.  Although everything is mixed in together, we can check to make sure the submission is an ask before proceeding.  Weirdly enough, when you post an ask with Tumblr, you're editing the already-created post.  We need to edit the ask with the body text, along with changing the ask's state to "published".  This will take the ask out of our inbox and push it onto our blog.  

Great, now that we have a basic understanding of what's going on, we can code it pretty easily.  

### Getting asks in our Inbox

This section isn't devoted on how to get fanmail, in case you were wondering.   But we will figure out what code-stuff we need to put in to make our bot read its inbox.

Navigate back to our `tumblr_bot.rb` file, and position your cursor right before the final `end`.  Add this line - 

```ruby
asks = client.submissions(USERNAME, limit: 3)
```

You'll notice that our first parameter is the all-uppercase `USERNAME`.  This isn't a typo, and you shouldn't substitute your own blog name here.  This is what's called a constant.  Because we need to access the username of the blog we will post to often, its good to define it in one place so we can change one line of code as opposed to 20.  At the top of our file, right after `require 'tumblr_client'`, we're going to define this constant.

```ruby
USERNAME = "doomybottest"       # or whatever your desired blog name is
```

You might also notice a new parameter, `limit: 3`.  This little bit of code lets us get 3 asks per inbox check.  That way we can respond to multiple messages in one go.

Let's also make one more temporary modification to our code by calling a `puts` on the `asks` variable, like so -

```ruby
asks = client.submissions(USERNAME, limit: 3)
puts asks
```

Before we test our code, we need to make sure something is in our bot's askbox.  For testing purposes, lets send our bot a few asks.

![](/images/publishing-asks-with-your-tumblr-bot-2/Capture.PNG)



Let's test our code.  Start a shell/command prompt in our project root folder, and type `ruby bin/bot.rb`.

> If you get a `dynamic constant assignment` error, check that your `USERNAME` constant is defined outside of the `post_a_thing` function.

If all goes well, the code should run without error, and you should get a response that looks like this - 

```ruby
{"posts"=>[{"blog_name"=>"doomybottest", "id"=>148370090189, "post_url"=>"https://www.tumblr.com/blog/doomybottest/submissions?148370090189", "slug"=>"will-i-ever-find-love", "type"=>"answer", "date"=>"2016-08-02 23:09:09 GMT", "timestamp"=>1470179349, "state"=>"submission", "format"=>"html", "reblog_key"=>"eneADWgq", "tags"=>[], "short_url"=>"https://tmblr.co/ZrmIvi2ABY_ZD", "summary"=>"Will I ever find love?", "recommended_source"=>nil, "recommended_color"=>nil, "followed"=>true, "highlighted"=>[], "liked"=>false, "note_count"=>0, "asking_name"=>"doomy", "asking_url"=>"http://doomy.me/", "question"=>"Will I ever find love?", "answer"=>"", "reblog"=>{"tree_html"=>"", "comment"=>""}, "trail"=>[], "can_send_in_message"=>false, "can_reply"=>true}, 
```

Yikes - that's a lot of... stuff.  What we're looking at is called a hash, and it contains all of the information a single ask possesses.  Although it's a little difficult to explain without understanding [serialization](https://en.wikipedia.org/wiki/Serialization), all we need to know is that all of our asks are stored in an array called `posts`.

So, let's modify our `client.submissions` line to get all `posts`.

```ruby
asks = client.submissions(USERNAME, limit: 3)['posts']
```

Now that we have an array of asks, we need to loop over each one (since we're getting 3 at a time).

Let's remove our `puts asks` line, and write this in it's place:

```ruby
asks.each do |ask|
    # only answer asks (don't worry about submissions or fanmail)
    if ask['type'] != 'answer'
        return
    end

    # temporarily print out our question
    puts ask['question']
end   
```

In completion, our code should currently look like this:

```ruby
require 'tumblr_client'
USERNAME = "doomybottest"

def post_a_thing

    # Authenticate via OAuth
    client = Tumblr::Client.new({
    :consumer_key => ENV['CONSUMER_KEY'],
    :consumer_secret => ENV['CONSUMER_SECRET'],
    :oauth_token => ENV['OAUTH_TOKEN'],
    :oauth_token_secret => ENV['OAUTH_TOKEN_SECRET']
    })

    asks = client.submissions(USERNAME, limit: 3)['posts']

    asks.each do |ask|
        # only answer asks (don't worry about submissions or fanmail)
        if ask['type'] != 'answer'
            return
        end

        # temporarily print out our question
        puts ask['question']
    end   

end
```


Let's go ahead and run our code again with `ruby bin/bot.rb`.  If all goes according to plan, we should get the text of the `question` we sent!

```bash
ruby bin/bot.rb
>> Will I ever find love?
```

Great, we completed step one on our chart.

### Steps 2, 3 and 4.

We can read asks that we got, but we can't really do anything with them.  Let's implement that now.  Let's replace the `puts ask['question']` line with the following code:

```ruby
response = "wow neato"
tags = "cool, stuff, vaporsoft"

client.edit(USERNAME,
    id: ask['id'],
    answer: response,
    state: 'published',
    tags: tags)
```

By calling `client.edit`, we do a few things to our ask.  We set its response to the variable `response` (creative!), set its tags to a variable called `tags`, and its state to `published`.  Let's go ahead and run our code with `ruby bin/bot.rb`.

After our code tells us it `posted a thing`, let's check our blog.  We should see our ask published!

 <div class="tumblr-post" data-href="https://embed.tumblr.com/embed/post/p0Y_HvspfCAUfl2VaRTZ2Q/148370955524" data-did="da39a3ee5e6b4b0d3255bfef95601890afd80709"><a href="http://doomybottest.tumblr.com/post/148370955524/will-i-ever-find-love">http://doomybottest.tumblr.com/post/148370955524/will-i-ever-find-love</a></div>  <script async src="https://secure.assets.tumblr.com/post.js"></script>

This is great and all, but answering "wow neato" to every ask isn't all that exciting, unless we have the url "passiveagressivebot" (which is up for grabs at the time of writing this, in case you were wondering).

Let's make our bot a little more contextual by greeting our askee by username, and then scrambling the question.  We can do this by editing our `response` variable.

```ruby
response = "Hi #{ask['asking_name']}, #{ask['question'].split(' ').shuffle.join(' ')}"
```

Lets deconstruct this very ruby statement - 

`ask['asking_name']` - gets us the blog name of the askee

`ask['question'].split(' ').shuffle.join(' ')` - gets us the question asked, and then mixes all the words around

Cool.  Try running your bot with `ruby bin/bot.rb` after sending your bot an ask.

 <div class="tumblr-post" data-href="https://embed.tumblr.com/embed/post/p0Y_HvspfCAUfl2VaRTZ2Q/148371473469" data-did="da39a3ee5e6b4b0d3255bfef95601890afd80709"><a href="http://doomybottest.tumblr.com/post/148371473469/hey-youre-a-big-idiot">http://doomybottest.tumblr.com/post/148371473469/hey-youre-a-big-idiot</a></div>  <script async src="https://secure.assets.tumblr.com/post.js"></script>

And... it works!  Fantastic.  If you're having trouble, here's the code in full.

```ruby
require 'tumblr_client'
USERNAME = "doomybottest"

def post_a_thing

    # Authenticate via OAuth
    client = Tumblr::Client.new({
    :consumer_key => ENV['CONSUMER_KEY'],
    :consumer_secret => ENV['CONSUMER_SECRET'],
    :oauth_token => ENV['OAUTH_TOKEN'],
    :oauth_token_secret => ENV['OAUTH_TOKEN_SECRET']
    })

    asks = client.submissions(USERNAME, limit: 3)['posts']

    asks.each do |ask|
        # only answer asks (don't worry about submissions or fanmail)
        if ask['type'] != 'answer'
            return
        end

        response = "Hi #{ask['asking_name']}, #{ask['question'].split(' ').shuffle.join(' ')}"
        tags = "cool, stuff, vaporsoft"

        client.edit(USERNAME,
                    id: ask['id'],
                    answer: response,
                    state: 'published',
                    tags: tags)

    end   

end
```

### Modifying for our own uses

We have a great base project for building more complicated bots.  All we need to do is modify the `response` and `tags` variables for totally different responses.  Ruby is really good at manipulating strings of text, so go out and explore!

<a href="/files/publishing-asks-with-your-tumblr-bot-2/publishing_asks_with_your_tumblr_bot.zip" class="button">Download the Whole Project</a>

### Conclusion
If you liked this tutorial, let me know by sharing this post, or adding a comment below.  Let me know if I should make more bot-oriented tutorials, too.  If you're feeling awfully nice, you can donate to me.

<a href="/donate" class="button">Donate</a>
