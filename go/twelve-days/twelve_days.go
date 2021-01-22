package twelve

var parts = []string{
	"a Partridge in a Pear Tree",
	"two Turtle Doves",
	"three French Hens",
	"four Calling Birds",
	"five Gold Rings",
	"six Geese-a-Laying",
	"seven Swans-a-Swimming",
	"eight Maids-a-Milking",
	"nine Ladies Dancing",
	"ten Lords-a-Leaping",
	"eleven Pipers Piping",
	"twelve Drummers Drumming",
}

var verseToDay = map[int]string{
	1:  "first",
	2:  "second",
	3:  "third",
	4:  "fourth",
	5:  "fifth",
	6:  "sixth",
	7:  "seventh",
	8:  "eighth",
	9:  "ninth",
	10: "tenth",
	11: "eleventh",
	12: "twelfth",
}

func intro(verse int) string {
	return "On the " + verseToDay[verse] + " day of Christmas my true love gave to me: "
}

func outro(verse int) string {
	var result string
	for i := verse; i > 0; i-- {
		result += parts[i-1]
		if i == 2 {
			result += ", and "
		} else if i == 1 {
			result += "."
		} else {
			result += ", "
		}
	}
	return result
}

// Verse returns a single verse from the twelve days of christmas song
func Verse(verse int) string {
	return intro(verse) + outro(verse)
}

// Song returns the lyrics for the twelve days of christmas song
func Song() string {
	var song = ""
	for verse, _ := range verseToDay {
		song += Verse(verse) + "\n"
	}
	return song
}
