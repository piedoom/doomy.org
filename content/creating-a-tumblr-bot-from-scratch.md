+++
title = "Creating a Tumblr Bot from Scratch"
date = 2016-07-22T01:00:00.007Z
author = "doomy"

[taxonomies]
categories = ["tutorial"]

[extra]
tags = ["bot", "tumblr", "ruby"]
+++

Today I'm gonna show how to make the basis for a bot like my own [doomybot](http://bot.doomy.me/).  It's actually pretty easy!

### Project Outline

This is the kind of quality output we can expect from the bot we are creating today -

 <div class="tumblr-post" data-href="https://embed.tumblr.com/embed/post/p0Y_HvspfCAUfl2VaRTZ2Q/147783220089" data-did="da39a3ee5e6b4b0d3255bfef95601890afd80709"><a href="http://doomybottest.tumblr.com/post/147783220089/neutral-good-doctor-muffin">http://doomybottest.tumblr.com/post/147783220089/neutral-good-doctor-muffin</a></div>  <script async src="https://secure.assets.tumblr.com/post.js"></script>

This set of tutorials will be broken up into several different components.  In this one, We'll get up-and-running with the Tumblr API and make a very simple bot that posts random sentences.  In the future, we'll outline how to answer asks, reblog posts, and do more advanced robot-like things.

Programming is a super-broad topic that I can't really cover in one tutorial, so if you're interested in learning more Ruby, a great place to start is [Codecademy](http://codecademy.com).  In order to fully understand everything we're doing, a little programming knowledge is helpful.  However, if you're completely new to all of this, you should be able to follow along just fine anyways.

### Setting Up

We need a couple things to start with.  Setting up will be slightly different depending on whether you're on a Mac/*nix system or Windows.  I greatly prefer *nix systems for this sort of stuff to Windows, but it can be done.

### Installing Ruby

The first thing we need to do is install Ruby, the programming language we will use.

###### On Macs:

Follow the guide [here](http://usabilityetc.com/articles/ruby-on-mac-os-x-with-rvm/).  I don't own a Mac, so this isn't tested!  But it should work.  Basically, we're installing RVM and then Ruby.

###### On Unix / Linux Systems

On Linux and Unix systems, these commands can change based on your distribution.  But generally, you can follow [this guide](https://rvm.io/) on the RVM site.

```bash
gpg --keyserver hkp://keys.gnupg.net --recv-keys 409B6B1796C275462A1703113804BB82D39DC0E3
\curl -sSL https://get.rvm.io | bash -s stable
``` 

Once you've installed RVM, get a new shell and type in `rvm install ruby`.  If all goes well, you will be a proud owner of the Ruby language.  

### On Windows

Windows kind of sucks for Ruby, okay?  It's great for .NET stuff, and pretty good at Python, but Ruby on Windows has some problems.  My recommendation is to install [Virtualbox](https://www.virtualbox.org/wiki/Downloads) for Windows, and then install a distribution of Linux like [Ubuntu](http://www.ubuntu.com/desktop) onto Virtualbox.  After that, follow the steps for Unix / Linux Systems above.

Because it's not reasonable to ask beginners to learn a whole new operating system, a Windows option still exists if you're persistent!  [RubyInstaller for Windows](http://rubyinstaller.org/) will package everything up nicely for you.  Select to install `Ruby 2.3.1`.  Note that you'll also need to install the [DevKit](http://rubyinstaller.org/add-ons/devkit/).  I also will not cover Windows-specific examples below - for instance, the `cd` command line tool is equivalent to `dir` in Windows, so not everything will work copy-and-pasted.



When the installer asks where you'd like to install Ruby, make sure you check `Add Ruby executables to your PATH`.

### Setting Up our Environment

We need to install a couple more things to get up and running.  Open up a new console window (Terminal in Mac and Command Prompt in Windows).  Type the following.
```ruby
gem install bundle
```
`bundle` is our package manager.  It will keep track of all the software stuff we need in our project.

Lets also create a folder to store our project.  This can be anywhere, and can be named anything (although it's helpful to avoid spaces and special characters) - 

```bash
mkdir tumblr_bot
```

We also will need a text editor to write everything with.  If you have one you prefer, use it!  (I like vim), but if you're unsure, I recommend using [Visual Studio Code](https://code.visualstudio.com/).  It's pretty good and runs on almost anything.

### Getting started

Create a few new folders in your project directory.  They will be titled `bin` and `lib`.  Also, create a new file called `Gemfile` in your project root.

- **bin**: this is where we'll store the stuff that actually runs our bot.  
- **lib**: this is where we'll store all the code.
- **Gemfile**: this is where we'll store everything we'll need for the `bundle` command to work.  It keeps a list of what software packages our project needs to run.  This file is especially vital if we want to share our program with other people.  Note that `Gemfile` has NO file extension.  It's just `Gemfile`!, not `Gemfile.txt`.

Your folder structure should look something like this.

![](/images/creating-a-tumblr-bot-from-scratch/DUea7NG.jpg)

Lets open our `lib` folder and add a new file named whatever the root folder is called.  My root folder is named `tumblr_bot`, so I'm going to add a new file called `tumblr_bot.rb`.  The `.rb` extension lets us know this is a Ruby file.

> Note: it doesn't really matter what this file is called, but I'm sticking to a convention often used in the creation of Ruby packages.  This will make your life easier down the line if you decide to author more Ruby projects.

### Creating the base of our project

Let's open up `tumblr_bot.rb` in the text editor of our choice.  In this file, we're going to put the following - 
```ruby
puts 'jello world'
```

And that's it for now.  Lets run it!  Go back to your root directory, and run the following:

```bash
ruby ./lib/tumblr_bot.rb
```

If all goes well, we should receive this output:

```bash
jello world
```

`puts`, or Put String, is the way to echo stuff to the console in Ruby.

### Setting up our Tumblr Account

First, lets create a new sub-blog.  This is easy enough from the Tumblr dashboard.  Name it anything you like, but remember the name.  You can also use your main blog, but some people might get confused if they get a pile of gibberish all over their dashboards.  Or that might just be normal.  Who knows.

In order to use your Tumblr account with custom code, you'll need to obtain an API key.  Log into Tumblr, and visit [this link](https://www.tumblr.com/oauth/apps).  Click "Register Application".

You only need to fill out a few fields.

**Application Name:** Whatever you want!  Name it something more creative than "TestBot".

**Application Website:** If you're not a member of the elite few to own a website yet, you can just use your Tumblr blog URL.

**Application Description:** Again, whatever you want.

**Administrative contact email:** You should make this something you actually use just in case Tumblr needs to notify you about something.

**Default callback URL:** We won't be using this parameter for our application, but it's still required.  You can put any valid URI here.

![](/images/creating-a-tumblr-bot-from-scratch/qNL6Ova.jpg)

And that's it!  Submit the form, and you'll have a new thing.  Find your new app, and click the `Show OAuth Consumer Secret`.  It should look like this except with a bunch of random characters instead of `xxxxx`.

![](/images/creating-a-tumblr-bot-from-scratch/Xi2fFbM.jpg)

We have to do one more thing before we're ready to roll.  Visit the [Tumblr API Console](https://api.tumblr.com/console/calls/user/info) with your Consumer Key and Consumer Secret on hand.  Input them into their respective input boxes, and hit "Authenticate".

> Note: This content is super-sensitive.  Treat it like you would a password.  

After you give access, you should be presented with a page that looks like this:

![](/images/creating-a-tumblr-bot-from-scratch/E0iLOnq.jpg)

Navigate to the tab that conveniently says `Ruby`!  

Copy this entire code section into your clipboard.

### Adding Tumblr to our App

With the code in our clipboard, lets move back to our `tumblr_bot.rb` file.  We can delete everything we added to this file before, and replace it with the code we just got from Tumblr.  Our `tumblr_bot.rb` file should now look like this:

```ruby
# Authenticate via OAuth
client = Tumblr::Client.new({
  :consumer_key => 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',
  :consumer_secret => 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',
  :oauth_token => 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',
  :oauth_token_secret => 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxx'
})

# Make the request
client.info 
```

Great!  Lets try and run it.  We'll do the same thing as last time; navigate to our root directory, open a console, and type in the following command.

```bash
ruby ./lib/tumblr_bot.rb
```

If all goes to plan, we should get an error!  Woo.

```bash
lib/tumblr_bot.rb:2:in `<main>': uninitialized constant Tumblr (NameError)
```

This is because we need the official Tumblr Ruby package.  Remember when we created our `Gemfile` and installed `bundle`?  We're about to use both these tools.

Open up your `Gemfile` and add the following lines:

```ruby
source 'https://rubygems.org'
gem 'tumblr_client'
```

Save the file, and get back to the terminal opened up in your root directory.  Lets type this behemoth of a command:

```bash
bundle
```

Bundle is the package we installed earlier.  It looks through our `Gemfile` file and adds any necessary packages to our system.  This could take a minute or two to complete.

We now have the Tumblr Ruby package installed on our system, but we have to specifically tell our file that we need to use it.  Open up `tumblr_bot.rb` again, and add the following line to the top of the file:

```ruby
require 'tumblr_client'
```

We also need to modify the last line.  Change it from this

```ruby
client.info
```

to this

```ruby
puts client.info
```

If you remember from before, `puts` will output our item to a string we can read.  Without the `puts` statement, the line would execute silently.

Your entire `tumblr_bot.rb` should look like this now:

```ruby
require 'tumblr_client'

# Authenticate via OAuth
client = Tumblr::Client.new({
  :consumer_key => 'xxxxxxxxxxxxxxxxxxxxx',
  :consumer_secret => 'xxxxxxxxxxxxxxxxxxxxx',
  :oauth_token => 'xxxxxxxxxxxxxxxxxxxxx',
  :oauth_token_secret => 'xxxxxxxxxxxxxxxxxxxxx'
})

# Make the request
puts client.info 
```

> Tip: when a line is preceded by the `#` symbol, the line is a comment, meaning that code is not executed.  You can use this to write yourself little notes.

Lets get back to our root console, and type the following command yet again:

```bash
ruby ./lib/tumblr_bot.rb
```

Andddddd..... We should get a response!  A response with a bunch of weird characters and symbols, but some readable text.  If you do some reading, you should see some stuff that looks familiar.

```ruby
{"user"=>{"name"=>"doomy", "likes"=>36042, "following"=>464, "default_post_format"=>"html", "blogs"=>[{"title"=>"meme machine", "name"=>"doomy", "total_posts"=>43367, "posts"=>43367, "url"=>"http://doomy.me/", "updated"=>1469145521, "description"=>"hi im alexander\nI made doomybot\n19 /  design student / developer / composer / certified funnyman / and more \nask\nmy music tag\nsoundcloudamazon wishlist",
...
```

It's all jumbled up in a bunch of `{`s and `"`, but this is definitely my blog!  You should see stuff that belongs to your own blog.

This is the result of the `client.info` command on which we prepended `puts`.  The `client.info` command just gets info about us!  This is cool and all, but let's make it a little more bot-like.

### Posting a thing

Bots autopost stuff!  It's what they do best.  So lets implement the same functionality.  

Open up your `tumblr_bot.rb` file, and delete the line we just edited:

```ruby
puts client.info 
```

We're going to replace it with some new commands.  Copy and paste the following, and add it to the end of your file:

```ruby
client.text("YOUR_SUBBLOG_HERE", title: "Jello World", body: "I am the jello maestro.")
```

This will make our entire file look like this:

```ruby 
require 'tumblr_client'

# Authenticate via OAuth
client = Tumblr::Client.new({
  :consumer_key => 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',
  :consumer_secret => 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',
  :oauth_token => 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',
  :oauth_token_secret => 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx'
})

# Make the request
client.text("YOUR_SUBBLOG_HERE", title: "Jello World", body: "I am the jello maestro.")
```

So what does it all mean?

**client:** This is something called a `variable` in programming.  Basically, it takes the easy-to-remember word `client` and sets it to the value of a big long `Tumblr` code that we can easily use later.  

**client.text:** The Tumblr variable `client` we stored beforehand as a bunch of nifty functions we can use by appending a `.`.  The `text` function allows us to create a text post, believe it or not.

**all that stuff after the client.text thing:** The data after our `client.text` thing are just parameters (or settings) for the `.text` function.  These parameters are surrounded in parenthesis, and are separated by commas.  Here, we're setting the blog we want to post to, then the title of our post, and then the body text of our post.  Note that you HAVE to own the blog or subblog you post to.  Don't try to post with the `staff` account, it won't work (probably).

Alright!  Let's go ahead and run our code.  

```bash
ruby ./lib/tumblr_bot.rb
```

And... Nothing?  Well, we didn't `puts` anything out to the console, so the program should just run and quit when it finished. But let's visit the blog name we provided in the `client.text` function.  With any luck, it should look like this!

 <div class="tumblr-post" data-href="https://embed.tumblr.com/embed/post/p0Y_HvspfCAUfl2VaRTZ2Q/147782490324" data-did="bc2b8a25d2c3825549694d9cdf25d866b3ddbd4f"><a href="http://doomybottest.tumblr.com/post/147782490324/jello-world">http://doomybottest.tumblr.com/post/147782490324/jello-world</a></div>  <script async src="https://secure.assets.tumblr.com/post.js"></script>

Nice!  We just posted some stuff from a script we wrote.  But we can make it even *cooler*.

### Making our Bot Post Things That Make our Mom Worry that We're Doing Illegal Drugs

Let's create something fun - a bot that defines a real-world object, and then assigns an alignment to that object.
We're going to add a few lines of code.  Here's what the file will look like:

```ruby
require 'tumblr_client'

# Authenticate via OAuth
client = Tumblr::Client.new({
  :consumer_key => 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',
  :consumer_secret => 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',
  :oauth_token => 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',
  :oauth_token_secret => 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx'
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
client.text("doomybottest", title: sentence)
```

It looks like a lot of stuff, but it's not too hard!  We have four basic variables - `alignments`, `adjectives`, `nouns`, and `sentence`.  

- The `alignments` variable holds all the possibilities in the [alignments chart](https://en.wikipedia.org/wiki/Alignment_(Dungeons_%26_Dragons)).

- The `adjectives` variable holds a list of, well, adjectives.  You can fill these in with whatever you want.  It doesn't strictly have to be adjectives, but try to choose items that *sort of* make sense given the context.

- The `nouns` variable holds a list of words that we can place after an item from the `adjectives` list.

- The `sentence` variable puts the `alignments`, `adjectives`, and `nouns` selections in one string.

You might be wondering, what is this weird syntax?

```ruby
thing = ["stuff", "woah"]
```
First of all, we need to surround any text in quotes, like `"stuff"`.  If we just used plain-old `stuff`, ruby would think we're looking for a variable called `stuff`, when that doesn't exist!

Also when we have a variable that is going to hold more than one thing, we need to use something called an array.  To create an array, we surround everything in the array with square brackets, and separate items with a comma.

Secondly, what's up with the `sentence` variable?

```ruby
sentence = "#{alignments.sample}: #{adjectives.sample} #{nouns.sample}."
```
First, when we call the `.sample` (a function of each array), Ruby nicely gets a random item from our array!  So, `nouns.sample` gets one random noun from our list of nouns, like `Pineapple Ham`.

So - remember when I said all text needs to be surrounded with quotes?  Well, that's true!  But what happens if we need to get the value of a variable?  Let's look at the following example:

```ruby
my_variable = "really cool thing"

puts my_variable
puts "my_variable"
```

If we run this code, it'll give us the following output:

```bash
really cool thing
my_variable
```

That's because when we surround anything with quotes, Ruby will treat it as a literal value, instead of getting the variable value.  So, when we want to include a variable inside of quotes, we can use the funky `#{my_variable}` syntax.  This is helpful when we need to put extra words around our variable.

So this -

```ruby
my_variable = "really cool thing"

puts "I think that ice water is a #{my_variable}"
```

Outputs this!


```bash
I think that ice water is a really cool thing
```

The same thing is true for our `sentence` variable.  We're just adding extra symbols and punctuation so our post is formatted well. So this - 

```ruby
sentence = "#{alignments.sample}: #{adjectives.sample} #{nouns.sample}."  
```

Has the possible output of this - 

 <div class="tumblr-post" data-href="https://embed.tumblr.com/embed/post/p0Y_HvspfCAUfl2VaRTZ2Q/147783813784" data-did="da39a3ee5e6b4b0d3255bfef95601890afd80709"><a href="http://doomybottest.tumblr.com/post/147783813784/neutral-evil-pathetic-boy">http://doomybottest.tumblr.com/post/147783813784/neutral-evil-pathetic-boy</a></div>  <script async src="https://secure.assets.tumblr.com/post.js"></script>

### Running and Automating

So, if we take the code from above, and run it with our magic command, it'll create one blog for us, and then quit.
```bash
ruby ./lib/tumblr_bot.rb
```

This isn't really useful for bots, which usually post once every few hours.  In order to automate this, we need to create a few changes to our `tumblr_bot.rb` file.  It should look like this.

```ruby
require 'tumblr_client'

# this part is different!
def post_a_thing
    # Authenticate via OAuth
    client = Tumblr::Client.new({
      :consumer_key => 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',
      :consumer_secret => 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',
      :oauth_token => 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',
      :oauth_token_secret => 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx'
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
    client.text("doomybottest", title: sentence)
end
```

We've wrapped our program in something new.  This is called a function, and it basically labels a section of code so we can reuse it a bunch of times in different places.  The syntax for creating a new function is `def function_name` where `function_name` is the name that you want as a label.  This can be anything, but it shouldn't have spaces or begin with a number.  We mark the end of a function with the `end` keyword.

If we run our code, you'd notice that nothing happens.  That's because in order to run a function, we need to call it!  At the very end of our file (after the `end` line), add the following, and then re-run.

```ruby
post_a_thing
```

Cool, we've successfully called our `post_a_thing` function just by putting the name at the end of our file.  You should notice our bot posted something.  Go ahead and delete the `post_a_thing` line from your `tumblr_bot.rb` that you just made.

Time to create a new file!  Let's create a new file in our `bin` directory that we made earlier, and let's call it `bot.rb`.  Open the file in your editor, and add the following lines.

```ruby
require_relative '../lib/tumblr_bot.rb'

while (true)
    post_a_thing
    puts 'posted a thing'
    sleep 3600
end
```

Okay, there's a lot of new stuff here, so what does it do?  Well first, we have this line - 

```ruby
require_relative '../lib/tumblr_bot.rb'
```

That includes the file we were working on beforehand, `tumblr_bot.rb`.

We also wrapped all of our code in a `while` loop.  This makes it so our program doesn't exit after it posts one thing, it loops infinitely until we shut it down ourselves by exiting out of the terminal.

Inside of our `while` loop, we call the function we just made, `post_a_thing`.  We also use `puts` to tell us that we posted something successfully.  Finally, we call `sleep 3600` to delay the time until our bot makes a new post.  the `sleep` function takes in a number of seconds to wait - in this case, we wait an hour, as `60 minutes * 60 seconds = 3600`.  

Finally, lets test our program - 

```bash
ruby ./bin/bot.rb
```

With any luck, we should get a response that we `posted a thing`.  Cool.  And if we leave it running, it'll generate a new post every hour!

<a href="/files/creating-a-tumblr-bot-from-scratch/creating-a-tumblr-bot-from-scratch.zip" class="button">Download the Whole Project</a>

### Conclusion
If you liked this tutorial, let me know by sharing this post, or adding a comment below.  Let me know if I should make more bot-oriented tutorials, too.  If you're feeling awfully nice, you can donate to me.

<a href="https://www.coinbase.com/piedoom" class="button">Donate with Bitcoin</a>

<form action="https://www.paypal.com/cgi-bin/webscr" method="post" target="_top" >
<input type="hidden" name="cmd" value="_s-xclick">
<input type="hidden" name="hosted_button_id" value="KS9AT7KB9ZBN4">
<input type="image" src="http://i.imgur.com/FR3Q7x3.jpg" border="0" name="submit" alt="PayPal - The safer, easier way to pay online!" style="width: auto; border: none; margin: 0; padding: 0;">
<img alt="" border="0" src="https://www.paypalobjects.com/en_US/i/scr/pixel.gif">
</form>
