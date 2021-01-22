package gigasecond

import (
	"math"
	"time"
)

// AddGigasecond adds 1 Gigasecond to a time
func AddGigasecond(t time.Time) time.Time {
	return time.Unix(t.Unix()+int64(math.Pow(10, 9)), 0)
}
