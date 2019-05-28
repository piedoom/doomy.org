+++
title = "Building an authenticated web API wrapper with Crystal"
date = 2016-09-20T01:00:00.007Z
author = "doomy"

[taxonomies]
categories = ["tutorial"]

[extra]
tags = ["programming", "crystal"]
+++

[Crystal](https://crystal-lang.org/) is an up-and-coming language very similar to Ruby, but compiled.  It's still very young, and it's changing all the time, but I've been having some fun playing around with it.  I started writing an [API wrapper for Tumblr](https://github.com/piedoom/tumblr-crystal) (which has basically become my "Hello World" now...).  Unfortunately, the documentation for some parts of Crystal doesn't yet exist, so I learned a ton from the community.  Because there aren't really any tutorials on how to do this sort of thing yet, I'd like to share what I've learned.

Big special thanks to [Asterite](https://github.com/asterite) and the group [Gitter chat](https://gitter.im/crystal-lang/crystal) for all of their help!

### Preface

This tutorial is for beginner / moderate skill level with programming.  If you know Ruby or even Python, you'll find this super-easy.  If you're stuck on something, send me a message and I'll try to clear it up!

### Installing

Because it's very possible that you've just heard of Crystal for the first time, you're probably going to need to install the library.  Detailed instructions on how to do this are available on the [Crystal docs website.](https://crystal-lang.org/docs/installation/index.html)

> Note: As of September 2016, Crystal does *not* have strategy for installing on Windows computers.  If you don't have access to a *nix or MacOS machine, consider using a virtual machine.

### Creating a Crystal Project

Once installed, it's easy to create a new Crystal project.  Let's create ours and name it `twitter-wrapper-test`.

```cr
crystal init lib twitter-wrapper-test
```

This will create a few files we'll use to bootstrap our project for use with Shards.  Speaking of Shards... what are they?

### Getting Started with Shards

Shards is Crystal's dependency manager, similar to Rubygems, NPM, and NuGet.  Crystalshards is decentralized in a way, where adding a dependency is basically just adding a git URI.  As of now, however, almost all repositories are hosted on GitHub.  You can search them with [Fatih Kadir Akın's](https://github.com/f) excellent [Crystalshards.xyz](http://crystalshards.xyz/).

Let's open up our `shard.yml` (which is similar to a Ruby project's `Gemfile`).  

```yml
name: twitter-wrapper-test
version: 0.1.0

authors:
  - doomy <myemail@email.com>

license: MIT
```

When a project is created, it initializes some basic information like the license, authors, and version.  Change these to your liking.  For this project, we don't need any dependencies.  Everything we need is built into the standard library!  (Awesome, right?)

### Getting started with Twitter

Although I'm using Twitter, feel free to follow along with another service as you see fit.  Keep in mind, however, that I'm using OAuth 1.0 in this tutorial.  You can use 2.0 with Crystal, but the process is a bit different.

Anyways, let's create a new Twitter application.  Sign in or create your Twitter account, and then click [here](https://apps.twitter.com/app/new) to generate a new app.  Don't worry about the callback URL for now, we don't need it.  Once that's finished, go to your new application's page, and click the "Keys and Access Tokens" tab.  At the bottom, click "Create my access token".  Great!  This will create everything we need to test our API wrapper - a `Consumer Key`, `Consumer Secret`, `Access Token Secret`, and `Access Token`.

### Setting up some environment variables

These keys we've created are **extremely** sensitive and should be treated as you would a password.  This means that hardcoding them in our new Crystal application is a huge no-no.  Because we're using a *nix system, we can set an environment variable.  Open up your shell's `.rc` file (in most cases, `~/.bashrc`) and append the following lines:

```bash
export TWITTER_CONSUMER_KEY="your-key-here"
export TWITTER_CONSUMER_SECRET="your-key-here"
export TWITTER_ACCESS_TOKEN="your-key-here"
export TWITTER_ACCESS_TOKEN_SECRET="your-key-here"
``` 

Where `your-key-here` is replaced with the corresponding value from your Twitter app.  Save and close your editor, and then open up a new terminal, or type `source ~/.[your shell]rc` (most likely `source ~/.bashrc`).

> Do we have to name our variables `TWITTER_CONSUMER_KEY`?

No, not really.  You can name these variables anything you like, but `TWITTER_CONSUMER_KEY` offers a lot of clarity!

### Building our wrapper

Time to do some coding.  Open up our `src/twitter-wrapper-test/` directory, and create a new file called `client.cr`.  Before we write any code, let's explain *how* and *why* we're going to structure the project like we are.  

##### Static Classes vs. Instantiation

We're going to instantiate the `client` class in order to perform requests.  Although we could definitely use a `static` class, it would have to be stateful and remember OAuth credentials.  Stateful static classes aren't very clear when implemented, so we're going to simply instatiate a new `client` object.

### Setting up an HTTP object with OAuth

Crystal provides OAuth support in its standard library.  Neat!  Let's open up our `client.cr` file and add a few `require` statements.  Let's also make a `module` that's the same name as our library.  (Notice that our `init` automatically created `Twitter::Wrapper::Test` because of the dashes.  This actually should probably be `TwitterWrapperTest`, so feel free to change all occurrences if you'd like.  For simplicity's sake, I'll just use what it generated).

```ruby
require "oauth"
require "http/client"
require "uri"

module Twitter::Wrapper::Test
  
end
```

Inside our module, we're going to create a new client class, with an initialize.

```ruby
class Client

  Host = "api.twitter.com"

  # create a new client with oauth support
  def initialize(consumer_key, consumer_secret, oauth_token, oauth_token_secret)

    # create our OAuth consumer and token with the built in library!
    consumer = OAuth::Consumer.new(Host, consumer_key, consumer_secret)
    token = OAuth::AccessToken.new(oauth_token, oauth_token_secret)

    # create an instance of the HTTP client using HTTPS
    @http_client = HTTP::Client.new(Host, tls: true, port: 443)

    # use the `authenticate` method on our consumer variable to authenticate our HTTP client with each request.  Neat!
    consumer.authenticate(@http_client, token)
  end
end
```

And that's just about it!  One of the most painless OAuth implementations in history :)

### Getting information from an endpoint

Twitter provides multiple endpoints for fetching JSON data.  For this tutorial, we're just going to search for some tweets.  You can read more about it's specification [here](https://dev.twitter.com/rest/reference/get/search/tweets).  Twitter provides some awesome API docs.

In our client, let's add a new method called `tweet_search`.

```ruby
def tweet_search(query : String)
  response = get("/1.1/search/tweets.json", {"q" => query})
end
```

Simple enough, right?  Well, if we tried to run this right now, we'd get a compiler error because our `get` method doesn't yet exist.  Let's create that, and a few other useful methods in our client class.

```ruby
# generic function for getting JSON
private def get(path : String, params = {} of String => String)

  # add parameters to our string
  path += "?#{to_query_params(params)}"  unless params.empty?

  # finally, get our response
  response = @http_client.get(path)

  # handle the response properly and check for errors
  handle_response(response)
end

private def handle_response(response : HTTP::Client::Response)
  case response.status_code
  when 200..299
    response.body
  else
    raise "#{response.status_code} - #{response.body}"
  end
end

# returns a URL encoded string used for query parameters
private def to_query_params(params : Hash(String, String))
  HTTP::Params.build do |form_builder|
    params.each do |key, value|
      form_builder.add(key, value)
    end
  end
end
```

These methods are pretty self explanatory.  I also have to admit, I almost entirely ripped these from [sferik's source code](https://github.com/sferik/twitter-crystal/blob/master/src/twitter/rest/client.cr). There's some great examples in that repo if you like reading source.  Unfortunately, there's no documentation for it yet :(

### Testing

Now that we have everything we need to make a simple request, let's see if our library works.

Open up `src/twitter-wrapper-test.cr`.  Let's write some code in the module block.

> Note: Usually, you're NEVER going to want to write app-specific code inside of a `lib` package.  Currently, there is no REPL in Crystal, so projects can't really be tested without making another Crystal application that consumes your custom library.  In order to avoid doing this for simplicity sake, we can write stuff directly in our `twitter-wrapper-test.cr` file.  Just remember to delete it before sharing!

```rb
client = Client.new(
    ENV["TWITTER_CONSUMER_KEY"], 
    ENV["TWITTER_CONSUMER_SECRET"],
    ENV["TWITTER_ACCESS_TOKEN"],
    ENV["TWITTER_ACCESS_TOKEN_SECRET"])

puts client.tweet_search("@crystal-lang")
```

Basically, we're creating a new client with our OAuth environment variables, and then searching for tweets with the string "@crystal-lang".  Let's test our application.

`crystal src/twitter-wrapper-test.cr`

If all goes to plan, we should get a huge strin of JSON.  This means our API authentication is working properly.  

### Deseralizing JSON with Crystal

Crystal provides some awesome, magical ways to deseralize JSON easily.  So let's get started!  Create a new file, `src/twitter-wrapper-test/tweet.cr`.  

We need to deserialize JSON that looks like [this](https://dev.twitter.com/rest/reference/get/search/tweets) (scroll down to the bottom for JSON examples).  There's a LOT of different properties for each tweet.  Let's just keep it basic and parse the `text`, `id`, and `user` properties.

In our `tweet.cr`...

```ruby
require "json"

module Twitter::Wrapper::Test
  class Tweet
    JSON.mapping(
      text: {type: String},
      id: {type: Int64},
      user: {type: User}
    )
  end
end
```

The `JSON.mapping` macro is an easy way for Crystal to figure out what properties of an object will be accessible, and how to deserialize/serialize the object to and from JSON.   

You might notice that our `user` property has a type that isn't already defined.  We need to make a new `User` class.  Create a new file as `src/twitter-wrapper-test/user.cr`. Again, there's a ton of properties given to us over JSON, so let's just take a few - `id`, `description`, and `screen_name`.

In our `user.cr`...

```ruby
require "json"

module Twitter::Wrapper::Test
  class User
    JSON.mapping(
      description: {type: String},
      name: {type: String, key: "screen_name"},
      id: {type: Int64}
    )
  end
end
```

For our `name` property, I passed in a `key` parameter with the value of "screen_name".  You can use this whenever your property name doesn't match up with the JSON 

### Finishing Up

Let's reopen our `client.cr` file so we can use our new JSON enabled classes.  At the end of our `tweet_search` method, add the following:

```ruby
return Array(Tweet).from_json(response, root: "statuses")
```

This will tell Crystal to use our JSON mappings to get an Array of Tweet objects in the root "statuses" (as defined in our JSON).

Let's switch back to our `twitter-wrapper-test.cr` file and replace the `puts ...` line with the following:

```ruby
tweets = client.tweet_search("@crystal-lang")
tweets.each do |tweet|
  puts "#{tweet.user.name} - #{tweet.text} \n\n"
end
```

And run `crystal src/twitter-wrapper-test.cr`

We should get this!

```
GaryAsh1969 - @burgerbecky i’ve been tracking this https://t.co/dMAPThdMuH since using Ruby 

ysbaddaden - I uploaded the long overdue FreeBSD tarballs for Crystal 0.19.0 up to 0.19.2 https://t.co/vTACMU98Mv 

piedoomy - RT @CrystalLanguage: Fund Crystal and help it become production-ready! https://t.co/AeBEGvw2TW 

MattStudies - Great post from the Crystal Team https://t.co/OgczM0lcEN 

Benchmarks are so frustratingly hard. 

versiontracker - Crystal 0.19.2 released. https://t.co/4NXtqAh1Lc #compiler #ruby #developers #programming_language #c https://t.co/OrZxr8Rxwv 
```

Cool.

### For the future & Conclusion

This is a very simple wrapper, and it doesn't implement everything Twitter has to offer.  Feel free to fully implement your client and push it to GitHub as a shard!

Crystal is constantly changing, so if you notice an issue with any of this code, please let me know so I can update it.  I am also just learning Crystal, so if you have an idea on how to do something better, please let me know.

### Donate

In lieu of donating to me, please consider funding Crystal on Bountysource so they can become production ready!

[![Bountysource](https://api.bountysource.com/badge/team?team_id=89730&style=raised)](https://www.bountysource.com/teams/crystal-lang/fundraisers/702-crystal-language)

Also, have my this Crystal Cat I made.

![](/images/building-an-authenticated-web-api-wrapper-with-crystal/crystalcat-02.png)
