+++
title = "Setting up RVM with Cygwin"
date = 2016-08-03T01:00:00.007Z
author = "doomy"

[taxonomies]
tags = ["ruby"]
+++

### Why?

If you're on Windows, many of my Ruby tutorials might not work as expected.  Cygwin is a terminal emulator for Windows that gives it the abilities of Linux / Unix.

RVM, or the Ruby Version Manager, is a really nice way of setting up Ruby on any system.  With a few lines, you can install multiple versions of Ruby.

### What you'll need

I'll assume that you have a working install of Cygwin.  If you don't, it's as simple as any other Windows setup.  You can download [Cygwin here](https://cygwin.com/install.html).

### Installing RVM and Ruby

Getting RVM up and running is pretty simple once you have Cygwin set up.  Simply open a new console and type

```bash
gpg --keyserver hkp://keys.gnupg.net --recv-keys 409B6B1796C275462A1703113804BB82D39DC0E3
```

And then 

```bash
\curl -sSL https://get.rvm.io | bash -s stable
```

If all goes to plan, you'll get a success message.  Close your current Cygwin terminal and open a new one to reload settings.  Then, type the following to install Ruby.

```bash
rvm install ruby
```

This might take a while.  Once it finishes, we'll type in `ruby -v` to make sure Ruby installed properly.

```bash
ruby -v
>> ruby 2.3.0p0 (2015-12-25 revision 53290) [x86_64-cygwin]
```


We'll also probably want the `bundle` gem so we can bulk-install Ruby packages.

```bash
gem install bundle
```

And that's it!

### Help!  I'm getting errors!

##### It fails when I do `rvm install ruby`

Try to run this command first:

```bash
rvm requirements
```

This will try to install all the stuff RVM relies upon for installing Ruby.  If this fails, you'll probably see an error similar to `setup-x86_64.exe not found`.  This means you'll have to add the Cygwin installer to your `PATH` (A system variable that tells Windows where all the good executable stuff is).

We can edit this by searching "Environment Variables" in the Start Menu.

![](Untitled.png)

Go ahead and click that option.  It'll bring up a dialog.  We want to click the "Environment Variables" option on the bottom right.

![](Capture-1.PNG)

In the section titled "User variables for *username here*", scroll and look for a `Variable` named `Path`.  Double click it.

![](Untitled-1.png)

Add a `New` variable by clicking the button on the top right.  The value of this will be whatever folder contains your `setup-x86_64.exe` file.  In my case, this was `E:\Program Files\Cygwin`, but by default, Cygwin installs to `C:\Cygwin`.  Note that you shouldn't append the actual file to your `Path` entry. 

CORRECT
```bash
C:\Cygwin
```


INCORRECT
```bash
C:\Cygwin\setup-x86_64.exe
```

Press `Okay` on both open dialogs, and then open a new instance of the Cygwin terminal.  If all goes right, `rvm requirements` should now succeed.