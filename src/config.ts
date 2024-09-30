import forumHappUrl from "./happs/forum.happ?url";
import relayHappUrl from "./happs/relay.happ?url";


export const BUNDLED_APPS = {
  forum: {
    name: 'Forum',
    description: 'A simple forum app with posts and comments.',
    url: forumHappUrl,
  },
  relay: {
    name: 'Relay',
    description: 'Chat app for the Volla ecosystem.',
    url: relayHappUrl,
  }
}