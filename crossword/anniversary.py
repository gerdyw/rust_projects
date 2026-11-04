import datetime

scores = [11000, 11201, 10651, 8801]

start = datetime.datetime(2026, 4, 1)
end = datetime.datetime(2026, 12, 1)

date_interval = end - start
date_range = date_interval.days

anniversary_day = sum(scores) % date_range + start.timetuple().tm_yday

anniversary = datetime.datetime(2026, 1, 1) + datetime.timedelta(days=anniversary_day)

print("=" * 50)
print("🎉 ANNIVERSARY DATE 🎉".center(50))
print("=" * 50)
print(f"\n{anniversary.strftime('%A, %B %d, %Y').upper()}".center(50))
print(f"\n{anniversary.strftime('%d/%m/%Y')}".center(50))
print("\n" + "=" * 50)
