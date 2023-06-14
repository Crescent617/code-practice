from music21 import *

# create a stream object to store the notes
stream1 = stream.Stream()

# create a list of notes and durations
notes = [
    "E5",
    "E5",
    "G5",
    "A5",
    "G5",
    "E5",
    "D5",
    "D5",
    "F5",
    "G5",
    "F5",
    "D5",
    "C5",
    "C5",
    "E5",
    "G5",
    "E5",
    "C5",
    "B4",
    "B4",
    "D5",
    "F5",
    "D5",
    "B4",
    "A4",
    "A4",
    "C5",
    "E5",
    "C5",
    "A4",
    "G4",
    "G4",
    "B4",
    "D5",
    "B4",
    "G4",
    # the last note has duration of one beat
    # so we don't need to specify it in the durations list
    # since it's the default value for duration.Duration()
    # and we can just append the note to the stream object
    # without specifying its duration
    "E4",
]

# durations = [1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 2, 2,
#              1, 1, 1, 1, 1, 2, 1, 1, 1, 2, 2,
#              1, 1, 1, 1, 1, 2, 1, 1, 1, 2]
durations = [
    0.5,
    0.5,
    1,
    1,
    0.5,
    0.5,
    1,
    0.5,
    0.5,
    1,
    1,
    1,
    0.5,
    0.5,
    0.5,
    0.5,
    0.5,
    1,
    0.5,
    0.5,
    0.5,
    1,
    1,
    0.5,
    0.5,
    0.5,
    0.5,
    0.5,
    1,
    0.5,
    0.5,
    0.5,
    1,
]

# add notes and durations to the stream object
for n, d in zip(notes, durations):
    note_ = note.Note(n)
    duration_ = duration.Duration(d)
    note_.duration = duration_
    stream1.append(note_)

# play the stream object
streamPlayer = midi.realtime.StreamPlayer(stream1)
streamPlayer.play()
