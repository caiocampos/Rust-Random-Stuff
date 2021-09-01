#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    frames: Vec<Frame>,
    frame: Frame,
    frame_count: u16,
    bonus_roll: u16,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            frames: Vec::new(),
            frame: Frame::new(),
            frame_count: 0,
            bonus_roll: 0,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        match (self.frame_count - self.bonus_roll == 10, pins) {
            (true, _) => Err(Error::GameComplete),
            (_, p) if p > self.frame.pins => Err(Error::NotEnoughPinsLeft),
            (_, p) => {
                let c = self.frame.count as usize;
                self.frame.pins -= p;
                self.frame.rolls[c] = p;
                self.frame.count += 1;
                if self.frame_count == 9 && self.frame.pins == 0 {
                    self.bonus_roll = 3 - self.frame.count;
                }
                if self.frame_count > 9 || self.frame.pins == 0 || self.frame.count == 2 {
                    self.frame_count += 1;
                    self.frames.push(self.frame);
                    self.frame = Frame::new();
                    if self.frame_count == 11 && p < 10 {
                        self.frame.pins -= p;
                    }
                }
                Ok(())
            }
        }
    }

    pub fn score(&self) -> Option<u16> {
        if self.frame_count - self.bonus_roll < 10 {
            None
        } else {
            let sum = self
                .frames
                .iter()
                .enumerate()
                .map(|(i, frame)| {
                    let mut val = frame.rolls[0] + frame.rolls[1];
                    if i < 9 && frame.pins == 0 {
                        let next = &self.frames[i + 1];
                        val += next.rolls[0];
                        if frame.count == 1 {
                            val += if next.count == 1 {
                                self.frames[i + 2].rolls[0]
                            } else {
                                next.rolls[1]
                            };
                        }
                    }
                    val
                })
                .sum();
            Some(sum)
        }
    }
}

#[derive(Copy, Clone)]
pub struct Frame {
    pins: u16,
    rolls: [u16; 2],
    count: u16,
}

impl Frame {
    pub fn new() -> Self {
        Frame {
            pins: 10,
            rolls: [0, 0],
            count: 0,
        }
    }
}
