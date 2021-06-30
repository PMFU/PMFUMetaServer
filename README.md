# PMFUMetaServer
This is a game routing server for the game that me and Milky are making. It is written in rust using ENet as the networking library.

# Goals
 1. It should be secure [should prevent: ddosing, ip leaking, cheat engine lobby exploits, and other possible threats] 
 2. It should be performant and scalable [should be async, multithreaded, and scalable] 
 3. It should be easy to use player side [should have: a pdx-mp style ID/password system, capability of checksum verification, hotjoins, leaves, etc]
