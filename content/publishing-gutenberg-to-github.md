+++
title = "Automatically publishing your Gutenberg project to Github pages"
date = "2017-12-16"
category = "tutorial"
tags = ["rust"]
+++

# Gutenberg & Static Site generators
If you're familiar with Github, you're most likely also aware of the Github Pages tool that allows users to publish static sites.  This is particularly useful for docs or a blog.  By default, Github uses Jekyll (A ruby based static site generator), but you can use whatever you want provided you have an `index.html` file in the root of a branch called `gh-pages`. 

I recently came across a project called [Gutenberg](https://www.getgutenberg.io/) by [Keats](https://github.com/Keats/gutenberg) and other contributors.  I was initially interested in the project because I like the Rust programming language, but Gutenberg has the additional value of having everything-you-need in a single executable.

This means that you don't need to fiddle with RVM or Python on your local machine to get started - you just need a [single executable](https://www.getgutenberg.io/documentation/getting-started/installation/).  Being said, Gutenberg is opinionated, so don't expect a billion preprocessors bundled.  You get Tera - a fine templating markup - as well as an SCSS preprocessor.  If for some reason you love to torture yourself with CoffeeScript or Babel, you'll have to do that stuff on your own. 

# Disclaimer
This post is not about how to use Gutenberg.  For that stuff, please check out Gutenberg's official docs.  They're super easy to read through and quick to execute.  [See it here.](https://www.getgutenberg.io/)

# Automating
Unlike Jekyll, Gutenberg is not run by Github.  In other words, if you push Gutenberg project files, they won't automatically build inside of Github and publish to the `gh-pages` branch.  However, we can use Travis CI to do this for us.  

I'm not going to explain exactly how to set up Travis as there are great documents already on that subject.  All you need to do is set up a new Github repository and enable it within the Travis dashboard. 

## Adding to your `.gitignore`
You don't want your project pushing the generated site to master accidentally if you do something like `gutenberg build` on master.  I recommend adding the following files to your `.gitignore`.  Keep in mind that this assumes that your project files are in the root directory.  For a static site generator *within* a code project, the structure may be different.

```
public
``` 

And that's it.  Gutenberg outputs everything to the `public` directory (unless of course you use the new [`--ouput-dir parameter`](https://github.com/Keats/gutenberg/pull/191))

## Setting up Travis
Before pushing anything, Travis needs a Github private access key in order to make changes to your repository.  If you're already logged in to your account, just click [here](https://github.com/settings/tokens) to go to your tokens page.  Otherwise, navigate to `Settings > Developer Settings > Personal Access Tokens`.  Generate a new token, and give it any description you'd like.  Under the "Select Scopes" section, give it `repo` permissions.  Click "Generate token" to finish up.

Your token will now be visible!  Copy it into your clipboard and head back to Travis.  Once on Travis, click on your project, and navigate to "Settings".  Scroll down to "Environment Variables" and input a name of `GH_TOKEN` with a value of your access token.  Make sure "Display value in build log" is off, and then click add.  Now Travis has access to your repository.  

### Encrypted Keys
Alternatively, you can put encrypted keys right into your `.travis.yml` file by using [this](https://docs.travis-ci.com/user/environment-variables/#Defining-encrypted-variables-in-.travis.yml) method.  Although this can be seen as more legible for other users, it requires Ruby and a special Travis gem.  I didn't have Ruby on my system, and I didn't feel like installing RVM, so I opted for just using the website.  Either is fine, and either will work!  If you do already have Ruby installed, I recommend the encrypted keys method.

## Setting up your settings
We're almost done.  We just need some scripts in a `.travis.yml` file to tell Travis what to do.

```yaml
before_script:
  - curl -s -L https://github.com/Keats/gutenberg/releases/download/v0.2.2/gutenberg-v0.2.2-x86_64-unknown-linux-gnu.tar.gz | sudo tar xvzf - -C /usr/local/bin

script:
  - gutenberg build

after_success: |
  [ $TRAVIS_BRANCH = master ] &&
  [ $TRAVIS_PULL_REQUEST = false ] &&
  gutenberg build &&
  sudo pip install ghp-import &&
  ghp-import -n public && 
  git push -fq https://${GH_TOKEN}@github.com/${TRAVIS_REPO_SLUG}.git gh-pages
```	

Lets take a look at that file again, but with comments.

```yaml
before_script:
  # download gutenberg's executable, unzip it, and add it to our binaries.  
  # This link isn't future proof!  By the time you read this article, a new
  # version may have been released.  Check the Gutenberg release page
  # on Github for more. 
  - curl -s -L https://github.com/Keats/gutenberg/releases/download/v0.2.2/gutenberg-v0.2.2-x86_64-unknown-linux-gnu.tar.gz | sudo tar xvzf - -C /usr/local/bin

script:
  - gutenberg build

after_success: |
  [ $TRAVIS_BRANCH = master ] &&
  [ $TRAVIS_PULL_REQUEST = false ] &&
  # This command builds our static site and automatically puts it in a directory 
  # called `public`, unless specified otherwise
  gutenberg build &&
  # install ghp-import - more on that below
  sudo pip install ghp-import &&
  ghp-import -n public && 
  # finally, push with that token that we set earlier
  git push -fq https://${GH_TOKEN}@github.com/${TRAVIS_REPO_SLUG}.git gh-pages
```	

If you already have a `.travis.yml` file for your project, just try merging the two files together manually.  

You can see we're using [ghp-import](https://github.com/davisp/ghp-import) which is just an easy way to publish Github pages.  

## Caveats 
Please note that `ghp-import` will *obliterate* anything in the `gh-pages` branch.  It won't just overwrite it, but completely destroy the branch and start new.  Keep this in mind if you need to preserve old versions for any reason.

Also, if you're using a custom domain name like I am, you'll need to specify that.  Instead of

```
ghp-import -n public
```

You'll need to write your domain name with the `-c` flag.  For instance, my website looks like this:

```
ghp-import -c vaporsoft.net -n public
```

# Conclusion
If all goes well, the next time you make a push on `master`, your Github Pages should update and be visible.  If it's not working, wait a minute or so.  Also make sure that the settings on your Github repo allow Github Pages.

# Thanks

Thanks to [Keats](https://vincent.is/) for authoring Gutenberg, as well as the many [contributors](https://github.com/Keats/gutenberg/graphs/contributors) 

Thanks to [t-rex tileserver](http://t-rex.tileserver.ch/) for providing an example CI file to go off of.

Thanks to [davisp](https://github.com/davisp) for authoring ghp-import, as well as the many [contributors](https://github.com/davisp/ghp-import/graphs/contributors)

# Theme
If you want a theme for Gutenberg, you can look at or use my own custom them that I use for this blog called feather.  [It's available here](https://github.com/piedoom/feather).  