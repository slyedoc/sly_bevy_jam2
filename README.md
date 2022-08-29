# sly_bevy_jam2 - Reactor

![Alt text](/docs/screenshot.png?raw=true "Reactor")

Theme is COMBINE

Playable here [Reactor](https://slyedoc.github.io/sly_bevy_jam2)

Created in 10 days for [Bevy Jam 2](https://itch.io/jam/bevy-jam-2)

## Post Mortem Notes

This was my first game jam ever, and bevy is amazing.  I have learned so much more in the last year than I ever did with Unity. 

I had no idea what the game was going to be till like day 6 and the "game" part I threw together the last 3 days, and it shows.  There are a hundred improvements I can think of, but still pretty happy with it.

My physics system needs a lot more work, biggest issues where:
 - New Type pattern is kind of a pain 
 - Need a transform gizmo for editing
 - Handles, need to change colliders over asp,
 - Spent 2 days trying to make a physics based character controller and had to many issues, this lead to me not using the physics much
 - Really need to add mesh to convex collider back
 - Transform Hierarchy, I don't support it thats a huge problem
 - Need sensors

I wasted so much time placing things though code, a simple scene save and basic editor to place items is a must.  It would have resulted in a better scenes in fraction of the time.  This will be top priority going forward.


[15.ai](https://15.ai) was amazing, will use this in the future, though I will most likely create a tool to automate its use.

[bevy_kira_audio](https://github.com/NiklasEi/bevy_kira_audio) worked well, few issues but most likely user error.

Second time using iyes_loopless, so much more useful than native states.

Used bevy_mod_outline for the first time, added nice user feedback.  Really a nice effect to let user know whats going on.

Had issues trying to use a few bevy community crates for lines and particles in wasm, need to come back to this.

I didn't use any shaders, was planning to, but things got way out of hand.
